// Updates section: mods with a newer compatible Modrinth version.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import '../../theme/m3_expressive.dart';

class UpdatesSection extends ConsumerStatefulWidget {
  const UpdatesSection({super.key, required this.instanceId, required this.onInstalled});

  final String instanceId;
  final VoidCallback onInstalled;

  @override
  ConsumerState<UpdatesSection> createState() => _UpdatesSectionState();
}

class _UpdatesSectionState extends ConsumerState<UpdatesSection> {
  List<ModUpdate>? _updates;
  bool _loading = false;

  Future<void> _check() async {
    final l10n = AppLocalizations.of(context);
    setState(() => _loading = true);
    try {
      final u = await ref
          .read(serviceProvider)
          .checkModUpdates(instanceId: widget.instanceId);
      if (!mounted) return;
      setState(() => _updates = u);
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    } finally {
      if (mounted) setState(() => _loading = false);
    }
  }

  Future<void> _updateAll() async {
    final l10n = AppLocalizations.of(context);
    final updates = _updates ?? const <ModUpdate>[];
    for (final u in updates) {
      try {
        await ref.read(serviceProvider).updateMod(
              instanceId: widget.instanceId,
              modId: u.installed.id,
              toVersionId: u.latest.versionId,
            );
      } on Object catch (e) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
        }
      }
    }
    ref.invalidate(instancesProvider);
    widget.onInstalled();
    if (mounted) setState(() => _updates = null);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final updates = _updates ?? const <ModUpdate>[];

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Text(l10n.modsUpdates,
                style: Theme.of(context).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700)),
            const Spacer(),
            TextButton.icon(
              onPressed: _loading ? null : _check,
              icon: const Icon(Icons.system_update_rounded, size: 18),
              label: Text(l10n.modsCheckUpdates),
            ),
            if (updates.isNotEmpty)
              FilledButton(
                onPressed: _updateAll,
                child: Text(l10n.modsUpdate),
              ),
          ],
        ),
        if (updates.isNotEmpty)
          for (final u in updates)
            Padding(
              padding: const EdgeInsets.only(bottom: 8),
              child: tonalCard(
                context: context,
                padding: const EdgeInsets.all(14),
                child: Row(
                  children: [
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(u.installed.name.isEmpty ? u.installed.fileName : u.installed.name,
                              style: const TextStyle(fontWeight: FontWeight.w600)),
                          Text(
                            '${u.installed.versionId ?? '?'} → ${u.latest.versionNumber}',
                            style: Theme.of(context).textTheme.bodySmall,
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
      ],
    );
  }
}