//! Mojang RSS news feed (task T4): parse → `NewsItem`, cache with a 1h TTL,
//! silently fall back to cache (or empty) on network failure.

use std::time::Duration;

use quick_xml::Reader;
use quick_xml::events::{BytesCData, Event};
use quick_xml::name::QName;


use yuhina_api::{NewsItem, YuhinaError};

use crate::store::Store;
use crate::YuhinaResult;

/// Default Mojang community-content RSS feed.
pub const DEFAULT_NEWS_RSS_URL: &str =
    "https://www.minecraft.net/en-us/feeds/community-content/rss";

const CACHE_TTL: Duration = Duration::from_secs(3600);
const SUMMARY_MAX_CHARS: usize = 200;

/// Fetches and caches Mojang news.
pub struct NewsService {
    store: Store,
    client: reqwest::Client,
    rss_url: String,
}

impl NewsService {
    pub fn new(store: Store, client: reqwest::Client) -> Self {
        Self::with_url(store, client, DEFAULT_NEWS_RSS_URL)
    }

    pub fn with_url(store: Store, client: reqwest::Client, rss_url: impl Into<String>) -> Self {
        Self {
            store,
            client,
            rss_url: rss_url.into(),
        }
    }

    /// Returns news, refreshing the cache when it is older than the TTL.
    /// On a network failure the cached copy is returned silently; when no
    /// cache exists an empty list is returned (never an error).
    pub async fn fetch_news(&self) -> YuhinaResult<Vec<NewsItem>> {
        if let Some(items) = self.fresh_cache()? {
            return Ok(items);
        }
        match self.fetch_remote().await {
            Ok(items) => {
                self.store_cache(&items)?;
                Ok(items)
            }
            Err(_) => self.read_cache(),
        }
    }

    /// Reads the cache only, without touching the network.
    pub fn get_news(&self) -> Vec<NewsItem> {
        self.read_cache().unwrap_or_default()
    }

    fn fresh_cache(&self) -> YuhinaResult<Option<Vec<NewsItem>>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        let fetched: Option<i64> = self.store.latest_news_fetched_at()?;
        match fetched {
            Some(f) if f >= 0 && now - f < CACHE_TTL.as_millis() as i64 => {
                Ok(Some(self.read_cache()?))
            }
            _ => Ok(None),
        }
    }

    fn read_cache(&self) -> YuhinaResult<Vec<NewsItem>> {
        self.store.list_news().map_err(internal)
    }

    fn store_cache(&self, items: &[NewsItem]) -> YuhinaResult<()> {
        self.store.replace_news(items).map_err(internal)
    }

    async fn fetch_remote(&self) -> YuhinaResult<Vec<NewsItem>> {
        let resp = self
            .client
            .get(&self.rss_url)
            .send()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(YuhinaError::network(format!(
                "RSS HTTP {}",
                resp.status().as_u16()
            )));
        }
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        let xml = String::from_utf8_lossy(&bytes);
        Ok(parse_rss(&xml))
    }
}

fn internal(e: impl std::fmt::Display) -> YuhinaError {
    YuhinaError::new(yuhina_api::YuhinaErrorKind::Internal, e.to_string())
}

fn qname<'a>(e: &'a QName) -> &'a str {
    e.as_ref()
}

/// Minimal RSS 2.0 parser: `<item><title|link|pubDate|description>`.
fn parse_rss(xml: &str) -> Vec<NewsItem> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut items = Vec::new();
    let mut in_item = false;
    let mut field: Option<String> = None;
    let mut cur = NewsItem {
        title: String::new(),
        url: String::new(),
        published: String::new(),
        summary: String::new(),
    };

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let n = e.name();
                let name = qname(&n);
                if name == "item" {
                    in_item = true;
                    cur.title.clear();
                    cur.url.clear();
                    cur.published.clear();
                    cur.summary.clear();
                } else if in_item && field.is_none() && is_field(name) {
                    field = Some(name.to_string());
                }
            }
            Ok(Event::Text(t)) => append_text(&mut cur, field.as_deref(), &t.html_content()),
            Ok(Event::CData(c)) => append_cdata(&mut cur, field.as_deref(), &c),
            Ok(Event::End(e)) => {
                let n = e.name();
                let name = qname(&n);
                if name == "item" {
                    if !cur.title.trim().is_empty() {
                        items.push(NewsItem {
                            title: std::mem::take(&mut cur.title),
                            url: std::mem::take(&mut cur.url),
                            published: std::mem::take(&mut cur.published),
                            summary: clean_summary(&cur.summary),
                        });
                    }
                    in_item = false;
                    field = None;
                } else if in_item && field.as_deref() == Some(name) {
                    field = None;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                tracing::warn!(%e, "rss parse error");
                break;
            }
            _ => {}
        }
    }
    items
}

fn is_field(name: &str) -> bool {
    matches!(name, "title" | "link" | "pubDate" | "description")
}

fn append_text(cur: &mut NewsItem, field: Option<&str>, text: &str) {
    match field {
        Some("title") => cur.title.push_str(text),
        Some("link") => cur.url.push_str(text),
        Some("pubDate") => cur.published.push_str(text),
        Some("description") => cur.summary.push_str(text),
        _ => {}
    }
}

fn append_cdata(cur: &mut NewsItem, field: Option<&str>, cdata: &BytesCData) {
    append_text(cur, field, cdata.as_ref());
}

/// Strips HTML tags, collapses whitespace and truncates the summary.
fn clean_summary(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut in_tag = false;
    for ch in raw.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    let joined: String = out.split_whitespace().collect::<Vec<_>>().join(" ");
    if joined.chars().count() > SUMMARY_MAX_CHARS {
        joined.chars().take(SUMMARY_MAX_CHARS).collect::<String>() + "…"
    } else {
        joined
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<?xml version="1.0"?>
<rss version="2.0">
  <channel>
    <title>Minecraft</title>
    <item>
      <title>Dungeons: New DLC!</title>
      <link>https://www.minecraft.net/en-us/article/new-dlc</link>
      <pubDate>Fri, 10 Jan 2025 12:00:00 GMT</pubDate>
      <description><![CDATA[<p>Announcing the <b>new DLC</b> &amp; more!</p>]]></description>
    </item>
    <item>
      <title>Snapshot 25w02a</title>
      <link>https://www.minecraft.net/en-us/article/snapshot-25w02a</link>
      <pubDate>Wed, 08 Jan 2025 12:00:00 GMT</pubDate>
      <description>Plain text summary</description>
    </item>
  </channel>
</rss>"#;

    #[test]
    fn parses_rss_items() {
        let items = parse_rss(SAMPLE);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Dungeons: New DLC!");
        assert_eq!(items[0].url, "https://www.minecraft.net/en-us/article/new-dlc");
        assert!(items[0].published.starts_with("Fri, 10 Jan 2025"));
        assert!(items[0].summary.contains("new DLC"));
        assert!(!items[0].summary.contains('<'));
        assert_eq!(items[1].title, "Snapshot 25w02a");
        assert_eq!(items[1].summary, "Plain text summary");
    }

    #[test]
    fn empty_and_garbage_feed() {
        assert!(parse_rss("").is_empty());
        assert!(parse_rss("not xml at all <<<>>>").is_empty());
    }

    #[test]
    fn summary_cleanup_truncates() {
        let long = format!("<p>{}</p>", "x".repeat(500));
        let out = clean_summary(&long);
        assert!(out.chars().count() <= SUMMARY_MAX_CHARS + 1);
        assert!(!out.contains('<'));
    }

    #[test]
    fn no_items_for_non_rss() {
        let items = parse_rss("<html><body>nothing</body></html>");
        assert!(items.is_empty());
    }
}