// Mojang news feed (cached by the Rust service; fetched on demand).

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../theme/m3_expressive.dart';

class NewsSection extends ConsumerWidget {
  const NewsSection({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final news = ref.watch(newsProvider);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Text(l10n.homeNews,
                style: Theme.of(context).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w700)),
            const Spacer(),
            TextButton.icon(
              onPressed: () => ref.read(serviceProvider).fetchNews(),
              icon: const Icon(Icons.refresh_rounded, size: 18),
              label: Text(l10n.commonRefresh),
            ),
          ],
        ),
        const SizedBox(height: 8),
        news.when(
          loading: () => const Center(child: Padding(padding: EdgeInsets.all(24), child: CircularProgressIndicator())),
          error: (_, _) => Text(l10n.homeNewsUnavailable),
          data: (items) {
            if (items.isEmpty) return Text(l10n.commonEmpty);
            return Column(
              children: [
                for (final item in items.take(6))
                  Padding(
                    padding: const EdgeInsets.only(bottom: 10),
                    child: tonalCard(
                      context: context,
                      padding: const EdgeInsets.all(16),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(item.title,
                              style: const TextStyle(fontWeight: FontWeight.w600)),
                          if (item.summary.isNotEmpty) ...[
                            const SizedBox(height: 4),
                            Text(item.summary,
                                maxLines: 2,
                                overflow: TextOverflow.ellipsis,
                                style: Theme.of(context).textTheme.bodySmall),
                          ],
                        ],
                      ),
                    ),
                  ),
              ],
            );
          },
        ),
      ],
    );
  }
}