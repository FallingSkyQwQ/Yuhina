// Mods page: installed list, conflicts banner, updates, search + file install.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import 'conflicts_banner.dart';
import 'mod_detail_sheet.dart';
import 'mod_search_page.dart';
import 'mod_tile.dart';
import 'updates_section.dart';

class ModsPage extends ConsumerStatefulWidget {
  const ModsPage({super.key, required this.instanceId});

  final String instanceId;

  @override
  ConsumerState<ModsPage> createState() => _ModsPageState();
}

class _ModsPageState extends ConsumerState<ModsPage> {
  List<ModUpdate>? _updates;
  List<ModConflict>? _conflicts;

  Future<void> _loadExtras() async {
    try {
      final updates =
          await ref.read(serviceProvider).checkModUpdates(instanceId: widget.instanceId);
      final conflicts =
          await ref.read(serviceProvider).checkModConflicts(instanceId: widget.instanceId);
      if (!mounted) return;
      setState(() {
        _updates = updates;
        _conflicts = conflicts;
      });
    } on Object catch (_) {
      // Conflict/update checks are best-effort; ignore failures.
    }
  }

  Future<void> _toggle(InstalledMod mod, bool enabled) async {
    final l10n = AppLocalizations.of(context);
    try {
      await ref.read(serviceProvider).setModEnabled(
            instanceId: widget.instanceId,
            modId: mod.id,
            enabled: enabled,
          );
      ref.invalidate(instancesProvider);
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    }
  }

  Future<void> _installFile() async {
    final l10n = AppLocalizations.of(context);
    final path = await showDialog<String>(
      context: context,
      builder: (ctx) {
        final c = TextEditingController();
        return AlertDialog(
          title: Text(l10n.modsInstallFile),
          content: TextField(
            controller: c,
            autofocus: true,
            decoration: const InputDecoration(hintText: '/path/to/mod.jar'),
          ),
          actions: [
            TextButton(onPressed: () => Navigator.pop(ctx), child: Text(l10n.commonCancel)),
            FilledButton(onPressed: () => Navigator.pop(ctx, c.text), child: Text(l10n.commonConfirm)),
          ],
        );
      },
    );
    if (path == null || path.trim().isEmpty) return;
    try {
      await ref
          .read(serviceProvider)
          .installModFile(instanceId: widget.instanceId, path: path.trim());
      ref.invalidate(instancesProvider);
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final mods = ref.watch(_modsProvider(widget.instanceId));
    final instance = ref.watch(_summaryProvider(widget.instanceId)).valueOrNull;

    return Scaffold(
      appBar: AppBar(
        title: Text(l10n.modsTitle),
        actions: [
          IconButton(
            tooltip: l10n.modsSearch,
            icon: const Icon(Icons.search_rounded),
            onPressed: () async {
              if (instance == null) return;
              await Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) => ModSearchPage(
                    instanceId: widget.instanceId,
                    mcVersion: instance.mcVersion,
                  ),
                ),
              );
              _loadExtras();
            },
          ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: () async {
          ref.invalidate(_modsProvider(widget.instanceId));
          await _loadExtras();
        },
        child: mods.when(
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (e, _) => Center(child: Text(localizeError(l10n, e))),
          data: (list) => ListView(
            padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
            children: [
              if (_conflicts != null) ...[
                ConflictsBanner(conflicts: _conflicts!),
                const SizedBox(height: 12),
              ],
              UpdatesSection(
                instanceId: widget.instanceId,
                onInstalled: () => setState(() => _updates = null),
              ),
              const SizedBox(height: 8),
              Row(
                children: [
                  Text('${list.length} ${l10n.modsTitle}',
                      style: Theme.of(context).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700)),
                  const Spacer(),
                  TextButton.icon(
                    onPressed: _installFile,
                    icon: const Icon(Icons.folder_open_rounded, size: 18),
                    label: Text(l10n.modsInstallFile),
                  ),
                ],
              ),
              if (list.isEmpty)
                Padding(padding: const EdgeInsets.all(24), child: Text(l10n.modsEmpty)),
              for (final m in list)
                Padding(
                  padding: const EdgeInsets.only(bottom: 8),
                  child: ModTile(
                    mod: m,
                    hasUpdate: _updates?.any((u) => u.installed.id == m.id) ?? false,
                    onToggle: (enabled) => _toggle(m, enabled),
                    onTap: () async {
                      final update = _updates?.where((u) => u.installed.id == m.id).firstOrNull;
                      await showModDetailSheet(
                        context,
                        ref,
                        instanceId: widget.instanceId,
                        mod: m,
                        update: update,
                      );
                      _loadExtras();
                    },
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}

final _modsProvider = FutureProvider.family<List<InstalledMod>, String>((ref, id) {
  return ref.watch(serviceProvider).listMods(instanceId: id);
});

final _summaryProvider = FutureProvider.family<InstanceSummary, String>((ref, id) async {
  final d = await ref.watch(serviceProvider).getInstance(id: id);
  return d.summary;
});