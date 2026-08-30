// Download center: task list (live progress), pause/resume/cancel/clear and
// modpack import entry.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import 'task_tile.dart';

class DownloadsPage extends ConsumerWidget {
  const DownloadsPage({super.key});

  Future<void> _action(
      BuildContext context, WidgetRef ref, AppLocalizations l10n, DownloadTask task) async {
    final svc = ref.read(serviceProvider);
    try {
      if (task.canPause) {
        await svc.pauseTask(id: task.id);
      } else if (task.state == DownloadState.paused || task.state == DownloadState.failed) {
        await svc.resumeTask(id: task.id);
      } else if (task.canCancel) {
        await svc.cancelTask(id: task.id);
      }
    } on Object catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
      }
    }
  }

  Future<void> _importModpack(BuildContext context, WidgetRef ref) async {
    final l10n = AppLocalizations.of(context);
    final path = await showDialog<String>(
      context: context,
      builder: (ctx) {
        final c = TextEditingController();
        return AlertDialog(
          title: Text(l10n.downloadsInstallModpack),
          content: TextField(controller: c, autofocus: true, decoration: const InputDecoration(hintText: '/path/to/xxx.mrpack')),
          actions: [
            TextButton(onPressed: () => Navigator.pop(ctx), child: Text(l10n.commonCancel)),
            FilledButton(onPressed: () => Navigator.pop(ctx, c.text), child: Text(l10n.commonConfirm)),
          ],
        );
      },
    );
    if (path == null || path.trim().isEmpty) return;
    try {
      final instance = await ref
          .read(serviceProvider)
          .importModpack(mrpackPath: path.trim(), name: '');
      ref.invalidate(instancesProvider);
      if (context.mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(SnackBar(content: Text('${instance.name} ✓')));
      }
    } on Object catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
      }
    }
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final tasks = ref.watch(downloadTasksProvider);

    return Scaffold(
      body: tasks.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (e, _) => Center(child: Text(localizeError(l10n, e))),
        data: (list) {
          if (list.isEmpty) {
            return Center(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(Icons.download_done_rounded, size: 56, color: Theme.of(context).colorScheme.outline),
                  const SizedBox(height: 12),
                  Text(l10n.downloadsEmpty),
                ],
              ),
            );
          }
          return ListView(
            padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
            children: [
              Align(
                alignment: Alignment.centerRight,
                child: TextButton.icon(
                  onPressed: () => ref.read(serviceProvider).clearFinishedTasks(),
                  icon: const Icon(Icons.delete_sweep_rounded, size: 18),
                  label: Text(l10n.downloadsClearFinished),
                ),
              ),
              for (final t in list)
                TaskTile(task: t, onAction: (task) => _action(context, ref, l10n, task)),
            ],
          );
        },
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () => _importModpack(context, ref),
        icon: const Icon(Icons.inventory_2_rounded),
        label: Text(l10n.downloadsInstallModpack),
      ),
    );
  }
}