// Instance library: responsive grid of instance cards + create/clone/delete.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import 'create_instance_sheet.dart';
import 'instance_card.dart';

class InstancesPage extends ConsumerWidget {
  const InstancesPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final instances = ref.watch(instancesProvider);

    return Scaffold(
      body: instances.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (e, _) => Center(child: Text(localizeError(l10n, e))),
        data: (list) {
          if (list.isEmpty) {
            return Center(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(Icons.extension_off_rounded, size: 56, color: Theme.of(context).colorScheme.outline),
                  const SizedBox(height: 12),
                  Text(l10n.instancesEmpty),
                  const SizedBox(height: 16),
                  FilledButton.icon(
                    onPressed: () => showCreateInstanceSheet(context, ref),
                    icon: const Icon(Icons.add_rounded),
                    label: Text(l10n.instancesNew),
                  ),
                ],
              ),
            );
          }
          return GridView.builder(
            padding: const EdgeInsets.fromLTRB(24, 8, 24, 96),
            gridDelegate: const SliverGridDelegateWithMaxCrossAxisExtent(
              maxCrossAxisExtent: 320,
              mainAxisSpacing: 16,
              crossAxisSpacing: 16,
              childAspectRatio: 0.92,
            ),
            itemCount: list.length,
            itemBuilder: (context, i) {
              final instance = list[i];
              return InstanceCard(
                instance: instance,
                onPlay: () => _play(context, ref, l10n, instance),
                onClone: () => _clone(context, ref, l10n, instance),
                onDelete: () => _confirmDelete(context, ref, l10n, instance),
              );
            },
          );
        },
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () => showCreateInstanceSheet(context, ref),
        icon: const Icon(Icons.add_rounded),
        label: Text(l10n.instancesNew),
      ),
    );
  }

  Future<void> _play(BuildContext context, WidgetRef ref, AppLocalizations l10n, InstanceSummary instance) async {
    final messenger = ScaffoldMessenger.of(context);
    try {
      await ref.read(serviceProvider).launchInstance(instanceId: instance.id);
      messenger.showSnackBar(SnackBar(content: Text('${instance.name} ▶')));
    } on Object catch (e) {
      messenger.showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    }
  }

  Future<void> _clone(BuildContext context, WidgetRef ref, AppLocalizations l10n, InstanceSummary instance) async {
    final name = await _promptText(context, l10n.instancesClone, '${instance.name} (2)');
    if (name == null || name.trim().isEmpty) return;
    try {
      await ref.read(serviceProvider).cloneInstance(id: instance.id, newName: name.trim());
    } on Object catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
      }
    }
  }

  Future<void> _confirmDelete(BuildContext context, WidgetRef ref, AppLocalizations l10n, InstanceSummary instance) async {
    final deleteFiles = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        title: Text(l10n.instancesDelete),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(l10n.instanceDeleteConfirm(instance.name)),
            const SizedBox(height: 8),
            CheckboxListTile(
              value: false,
              onChanged: (_) {},
              title: Text(l10n.instanceDeleteFiles),
            ),
          ],
        ),
        actions: [
          TextButton(onPressed: () => Navigator.pop(ctx, false), child: Text(l10n.commonCancel)),
          FilledButton(onPressed: () => Navigator.pop(ctx, true), child: Text(l10n.commonDelete)),
        ],
      ),
    );
    if (deleteFiles == null) return;
    try {
      await ref.read(serviceProvider).deleteInstance(id: instance.id, deleteFiles: deleteFiles);
    } on Object catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
      }
    }
  }

  Future<String?> _promptText(BuildContext context, String title, String initial) async {
    final controller = TextEditingController(text: initial);
    final result = await showDialog<String>(
      context: context,
      builder: (ctx) => AlertDialog(
        title: Text(title),
        content: TextField(controller: controller, autofocus: true),
        actions: [
          TextButton(onPressed: () => Navigator.pop(ctx), child: Text(AppLocalizations.of(ctx).commonCancel)),
          FilledButton(onPressed: () => Navigator.pop(ctx, controller.text), child: Text(AppLocalizations.of(ctx).commonConfirm)),
        ],
      ),
    );
    controller.dispose();
    return result;
  }
}