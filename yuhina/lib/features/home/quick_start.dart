// Quick launch card: picks the most recently-launched instance and offers a
// one-tap launch, plus a shortcut to the instance library.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import '../../theme/m3_expressive.dart';

class QuickStart extends ConsumerWidget {
  const QuickStart({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final instances = ref.watch(instancesProvider).valueOrNull ?? const [];
    final accounts = ref.watch(accountsProvider).valueOrNull ?? const [];
    final account = accounts.where((a) => a.isActive).firstOrNull;

    if (instances.isEmpty) {
      return tonalCard(
        context: context,
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Icon(Icons.rocket_launch_rounded, size: 40, color: Theme.of(context).colorScheme.primary),
            const SizedBox(height: 12),
            Text(l10n.instancesEmpty, style: Theme.of(context).textTheme.bodyLarge),
            const SizedBox(height: 12),
            FilledButton.icon(
              onPressed: () => context.go('/instances'),
              icon: const Icon(Icons.add_rounded),
              label: Text(l10n.instancesNew),
            ),
          ],
        ),
      );
    }

    instances.sort((a, b) => (b.lastLaunchedAt ?? BigInt.zero).compareTo(a.lastLaunchedAt ?? BigInt.zero));
    final target = instances.first;

    return tonalCard(
      context: context,
      padding: const EdgeInsets.all(24),
      child: Row(
        children: [
          CircleAvatar(
            radius: 30,
            backgroundColor: Theme.of(context).colorScheme.primaryContainer,
            child: Text(target.icon, style: const TextStyle(fontSize: 28)),
          ),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(l10n.homeQuickLaunch,
                    style: Theme.of(context).textTheme.labelLarge?.copyWith(
                          color: Theme.of(context).colorScheme.primary,
                          fontWeight: FontWeight.w700,
                        )),
                const SizedBox(height: 4),
                Text(target.name,
                    style: Theme.of(context).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w700)),
                const SizedBox(height: 2),
                Text(
                  '${target.mcVersion}${target.loader != null ? ' · ${loaderLabel(target.loader!.kind)} ${target.loader!.version}' : ''}',
                  style: Theme.of(context).textTheme.bodySmall,
                ),
              ],
            ),
          ),
          FilledButton.icon(
            onPressed: () => _launch(context, ref, l10n, target, account),
            icon: const Icon(Icons.play_arrow_rounded),
            label: Text(l10n.instancesPlay),
          ),
        ],
      ),
    );
  }

  String loaderLabel(LoaderKind kind) => switch (kind) {
        LoaderKind.forge => 'Forge',
        LoaderKind.fabric => 'Fabric',
        LoaderKind.neoForge => 'NeoForge',
        LoaderKind.quilt => 'Quilt',
      };

  Future<void> _launch(BuildContext context, WidgetRef ref, AppLocalizations l10n,
      InstanceSummary instance, Account? account) async {
    final messenger = ScaffoldMessenger.of(context);
    if (account == null) {
      messenger.showSnackBar(SnackBar(content: Text(l10n.errorNotLoggedIn)));
      return;
    }
    try {
      await ref.read(serviceProvider).launchInstance(instanceId: instance.id);
      messenger.showSnackBar(SnackBar(content: Text('${instance.name} ▶')));
    } on Object catch (e) {
      messenger.showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    }
  }
}