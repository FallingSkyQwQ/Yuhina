// Mod detail sheet: description, loaders/versions, update + remove actions.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

Future<void> showModDetailSheet(
  BuildContext context,
  WidgetRef ref, {
  required String instanceId,
  required InstalledMod mod,
  required ModUpdate? update,
}) {
  return showModalBottomSheet<void>(
    context: context,
    isScrollControlled: true,
    builder: (_) => ModDetailSheet(ref: ref, instanceId: instanceId, mod: mod, update: update),
  );
}

class ModDetailSheet extends ConsumerStatefulWidget {
  const ModDetailSheet({
    super.key,
    required this.ref,
    required this.instanceId,
    required this.mod,
    required this.update,
  });

  final WidgetRef ref;
  final String instanceId;
  final InstalledMod mod;
  final ModUpdate? update;

  @override
  ConsumerState<ModDetailSheet> createState() => _ModDetailSheetState();
}

class _ModDetailSheetState extends ConsumerState<ModDetailSheet> {
  bool _busy = false;

  Future<void> _update() async {
    final l10n = AppLocalizations.of(context);
    final update = widget.update;
    if (update == null) return;
    setState(() => _busy = true);
    try {
      await widget.ref.read(serviceProvider).updateMod(
            instanceId: widget.instanceId,
            modId: widget.mod.id,
            toVersionId: update.latest.versionId,
          );
      widget.ref.invalidate(instancesProvider);
      if (!mounted) return;
      Navigator.pop(context);
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  Future<void> _remove() async {
    final l10n = AppLocalizations.of(context);
    setState(() => _busy = true);
    try {
      await widget.ref.read(serviceProvider).deleteMod(
            instanceId: widget.instanceId,
            modId: widget.mod.id,
          );
      widget.ref.invalidate(instancesProvider);
      if (!mounted) return;
      Navigator.pop(context);
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final mod = widget.mod;
    final scheme = Theme.of(context).colorScheme;

    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 8, 20, 20),
      child: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                CircleAvatar(
                  backgroundColor: scheme.primaryContainer,
                  child: Icon(Icons.extension_rounded, color: scheme.onPrimaryContainer),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(mod.name.isEmpty ? mod.fileName : mod.name,
                          style: Theme.of(context).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700)),
                      Text(mod.fileName, style: Theme.of(context).textTheme.bodySmall),
                    ],
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            if (mod.description.isNotEmpty)
              Text(mod.description, style: Theme.of(context).textTheme.bodyMedium),
            const SizedBox(height: 8),
            Wrap(
              spacing: 8,
              children: [
                for (final l in mod.loaders) Chip(label: Text(l)),
                for (final v in mod.mcVersions) Chip(label: Text(v)),
              ],
            ),
            const SizedBox(height: 16),
            Row(
              children: [
                if (widget.update != null) ...[
                  Expanded(
                    child: FilledButton.icon(
                      onPressed: _busy ? null : _update,
                      icon: const Icon(Icons.system_update_alt_rounded),
                      label: Text(l10n.modsUpdate),
                    ),
                  ),
                  const SizedBox(width: 8),
                ],
                Expanded(
                  child: OutlinedButton.icon(
                    onPressed: _busy ? null : _remove,
                    icon: const Icon(Icons.delete_outline_rounded),
                    label: Text(l10n.modsRemove),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}