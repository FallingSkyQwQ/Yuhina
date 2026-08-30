//! Mojang news fetch + cache integration tests (task T4).

mod common;

use common::{MockConfig, MockServer};
use yuhina_download::NewsService;

const RSS: &str = r#"<?xml version="1.0"?>
<rss version="2.0">
  <channel>
    <title>Minecraft</title>
    <item>
      <title>First News</title>
      <link>https://www.minecraft.net/en-us/article/first</link>
      <pubDate>Fri, 10 Jan 2025 12:00:00 GMT</pubDate>
      <description>First summary</description>
    </item>
    <item>
      <title>Second News</title>
      <link>https://www.minecraft.net/en-us/article/second</link>
      <pubDate>Wed, 08 Jan 2025 12:00:00 GMT</pubDate>
      <description><![CDATA[<p>Second <b>summary</b></p>]]></description>
    </item>
  </channel>
</rss>"#;

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

#[tokio::test]
async fn fetch_parses_and_caches() {
    let server = MockServer::start(MockConfig {
        data: RSS.as_bytes().to_vec(),
        ..Default::default()
    });
    let db = yuhina_download::Store::in_memory().unwrap();
    let news = NewsService::with_url(db, client(), server.url("/rss"));

    let items = news.fetch_news().await.unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].title, "First News");
    assert_eq!(items[1].summary, "Second summary");

    // Second fetch hits the cache (TTL 1h) — no extra network request.
    let items2 = news.fetch_news().await.unwrap();
    assert_eq!(items2.len(), 2);
    assert_eq!(server.hit_count(), 1);

    // Cache read-only access works too.
    assert_eq!(news.get_news().len(), 2);
}

#[tokio::test]
async fn failure_falls_back_to_cache_silently() {
    let mut server = MockServer::start(MockConfig {
        data: RSS.as_bytes().to_vec(),
        ..Default::default()
    });
    let db = yuhina_download::Store::in_memory().unwrap();
    let news = NewsService::with_url(db, client(), server.url("/rss"));
    assert_eq!(news.fetch_news().await.unwrap().len(), 2);

    // Kill the server; the next fetch must still return the cached copy.
    server.shutdown();
    let items = news.fetch_news().await.unwrap();
    assert_eq!(items.len(), 2);
}

#[tokio::test]
async fn empty_cache_with_no_network_returns_empty_not_error() {
    let db = yuhina_download::Store::in_memory().unwrap();
    // Port 1: connection refused, no listener.
    let news = NewsService::with_url(db, client(), "http://127.0.0.1:1/rss");
    let items = news.fetch_news().await.unwrap();
    assert!(items.is_empty());
    assert!(news.get_news().is_empty());
}

#[tokio::test]
async fn server_error_falls_back_to_empty() {
    let server = MockServer::start(MockConfig {
        data: Vec::new(),
        fail_count: 999, // always 500
        ..Default::default()
    });
    let db = yuhina_download::Store::in_memory().unwrap();
    let news = NewsService::with_url(db, client(), server.url("/rss"));
    let items = news.fetch_news().await.unwrap();
    assert!(items.is_empty());
}

#[tokio::test]
async fn get_news_returns_cached_without_network() {
    let mut server = MockServer::start(MockConfig {
        data: RSS.as_bytes().to_vec(),
        ..Default::default()
    });
    let db = yuhina_download::Store::in_memory().unwrap();
    let news = NewsService::with_url(db, client(), server.url("/rss"));
    let _ = news.fetch_news().await.unwrap();
    server.shutdown();
    // get_news never touches the network.
    assert_eq!(news.get_news().len(), 2);
}
