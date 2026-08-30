// About tab: version + launcher self-update check.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../theme/m3_expressive.dart';

class AboutTab extends ConsumerWidget {
  const AboutTab({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final scheme = Theme.of(context).colorScheme;

    return ListView(
      padding: const EdgeInsets.all(20),
      children: [
        tonalCard(
          context: context,
          padding: const EdgeInsets.all(24),
          child: Column(
            children: [
              CircleAvatar(
                radius: 36,
                backgroundColor: scheme.primaryContainer,
                child: Icon(Icons.videogame_asset_rounded, size: 40, color: scheme.onPrimaryContainer),
              ),
              const SizedBox(height: 12),
              Text('Yuhina',
                  style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontWeight: FontWeight.w800)),
              const SizedBox(height: 4),
              Text(l10n.settingsAboutText('0.1.0+1'),
                  style: Theme.of(context).textTheme.bodySmall),
              const SizedBox(height: 16),
              _UpdateCheck(ref: ref),
            ],
          ),
        ),
      ],
    );
  }
}

class _UpdateCheck extends ConsumerWidget {
  const _UpdateCheck({required this.ref});

  final WidgetRef ref;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    return FutureBuilder<String?>(
      future: ref.read(serviceProvider).checkLauncherUpdate(),
      builder: (context, snap) {
        if (snap.connectionState == ConnectionState.waiting) {
          return const SizedBox(height: 24, width: 24, child: CircularProgressIndicator(strokeWidth: 2));
        }
        if (snap.hasError) {
          return Text(l10n.settingsUpToDate);
        }
        final version = snap.data;
        return Text(
          version != null
              ? l10n.settingsUpdateAvailable(version)
              : l10n.settingsUpToDate,
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                color: version != null ? Theme.of(context).colorScheme.primary : null,
                fontWeight: FontWeight.w600,
              ),
        );
      },
    );
  }
}