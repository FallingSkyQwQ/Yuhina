// Java tab: list runtimes, scan system, add manual path, download, remove.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

class JavaTab extends ConsumerWidget {
  const JavaTab({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final runtimes = ref.watch(javaRuntimesProvider);

    return ListView(
      padding: const EdgeInsets.all(20),
      children: [
        Wrap(
          spacing: 8,
          runSpacing: 8,
          children: [
            FilledButton.icon(
              onPressed: () async {
                try {
                  await ref.read(serviceProvider).scanSystemJava();
                  ref.invalidate(javaRuntimesProvider);
                } on Object catch (e) {
                  if (context.mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
                  }
                }
              },
              icon: const Icon(Icons.radar_rounded),
              label: Text(l10n.settingsScanJava),
            ),
            FilledButton.tonalIcon(
              onPressed: () => _addManual(context, ref, l10n),
              icon: const Icon(Icons.add_rounded),
              label: Text(l10n.settingsAddManualJava),
            ),
            FilledButton.tonalIcon(
              onPressed: () => _download(context, ref, l10n),
              icon: const Icon(Icons.download_rounded),
              label: Text(l10n.settingsDownloadJava),
            ),
          ],
        ),
        const SizedBox(height: 16),
        runtimes.when(
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (e, _) => Text(localizeError(l10n, e)),
          data: (list) {
            if (list.isEmpty) return Text(l10n.commonEmpty);
            return Column(
              children: [
                for (final j in list)
                  Card.outlined(
                    margin: const EdgeInsets.only(bottom: 10),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(18)),
                    child: ListTile(
                      leading: CircleAvatar(
                        backgroundColor: Theme.of(context).colorScheme.primaryContainer,
                        child: Text('${j.major}',
                            style: TextStyle(fontWeight: FontWeight.w700, color: Theme.of(context).colorScheme.onPrimaryContainer)),
                      ),
                      title: Text('${j.vendor} ${j.version}'),
                      subtitle: Text('${_sourceLabel(j.source)} · ${j.path}', maxLines: 1, overflow: TextOverflow.ellipsis),
                      trailing: IconButton(
                        tooltip: l10n.settingsRemoveJava,
                        icon: const Icon(Icons.delete_outline_rounded),
                        onPressed: () async {
                          try {
                            await ref.read(serviceProvider).removeJava(id: j.id);
                            ref.invalidate(javaRuntimesProvider);
                          } on Object catch (e) {
                            if (context.mounted) {
                              ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
                            }
                          }
                        },
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

  String _sourceLabel(JavaSource s) => switch (s) {
        JavaSource.bundled => 'Bundled',
        JavaSource.system => 'System',
        JavaSource.manual => 'Manual',
      };

  Future<void> _addManual(BuildContext context, WidgetRef ref, AppLocalizations l10n) async {
    final path = await _promptPath(context, l10n, '/usr/lib/jvm/.../bin/java');
    if (path == null || path.trim().isEmpty) return;
    try {
      await ref.read(serviceProvider).addManualJava(path: path.trim());
      ref.invalidate(javaRuntimesProvider);
    } on Object catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
      }
    }
  }

  Future<void> _download(BuildContext context, WidgetRef ref, AppLocalizations l10n) async {
    final major = await showDialog<int>(
      context: context,
      builder: (ctx) => SimpleDialog(
        title: Text(l10n.settingsDownloadJava),
        children: [
          for (final m in [8, 11, 17, 21])
            SimpleDialogOption(onPressed: () => Navigator.pop(ctx, m), child: Text('Java $m')),
        ],
      ),
    );
    if (major == null) return;
    try {
      await ref.read(serviceProvider).installJava(major: major);
      ref.invalidate(javaRuntimesProvider);
    } on Object catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
      }
    }
  }

  Future<String?> _promptPath(BuildContext context, AppLocalizations l10n, String hint) async {
    final c = TextEditingController();
    return showDialog<String>(
      context: context,
      builder: (ctx) => AlertDialog(
        title: Text(l10n.settingsAddManualJava),
        content: TextField(controller: c, autofocus: true, decoration: InputDecoration(hintText: hint)),
        actions: [
          TextButton(onPressed: () => Navigator.pop(ctx), child: Text(l10n.commonCancel)),
          FilledButton(onPressed: () => Navigator.pop(ctx, c.text), child: Text(l10n.commonConfirm)),
        ],
      ),
    );
  }
}