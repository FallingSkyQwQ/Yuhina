// Modrinth search page: query, results, version selection, install.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../core/format.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

class ModSearchPage extends ConsumerStatefulWidget {
  const ModSearchPage({super.key, required this.instanceId, required this.mcVersion});

  final String instanceId;
  final String mcVersion;

  @override
  ConsumerState<ModSearchPage> createState() => _ModSearchPageState();
}

class _ModSearchPageState extends ConsumerState<ModSearchPage> {
  final _query = TextEditingController();
  SearchResult? _result;
  bool _loading = false;
  String? _error;

  @override
  void dispose() {
    _query.dispose();
    super.dispose();
  }

  Future<void> _search({int index = 0}) async {
    final l10n = AppLocalizations.of(context);
    setState(() {
      _loading = true;
      _error = null;
    });
    try {
      final r = await ref.read(serviceProvider).searchMods(
            query: _query.text.trim(),
            loaders: const [],
            gameVersions: [widget.mcVersion],
            index: index,
            limit: 20,
          );
      if (!mounted) return;
      setState(() => _result = r);
    } on Object catch (e) {
      if (!mounted) return;
      setState(() => _error = localizeError(l10n, e));
    } finally {
      if (mounted) setState(() => _loading = false);
    }
  }

  Future<void> _install(ModrinthProject project) async {
    final l10n = AppLocalizations.of(context);
    final versions = await ref.read(serviceProvider).listModVersions(
          projectId: project.projectId,
          loaders: const [],
          gameVersions: [widget.mcVersion],
        );
    if (!mounted || versions.isEmpty) return;
    final version = await showModalBottomSheet<ModrinthVersion>(
      context: context,
      builder: (ctx) => ListView(
        shrinkWrap: true,
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Text(project.title,
                style: Theme.of(ctx).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700)),
          ),
          for (final v in versions.take(10))
            ListTile(
              leading: const Icon(Icons.tag_rounded),
              title: Text(v.versionNumber),
              subtitle: Text(v.name),
              trailing: Text(formatBytes(v.files.isNotEmpty ? v.files.first.size.toInt() : 0)),
              onTap: () => Navigator.pop(ctx, v),
            ),
        ],
      ),
    );
    if (version == null) return;
    try {
      await ref.read(serviceProvider).installMod(
            instanceId: widget.instanceId,
            projectId: project.projectId,
            versionId: version.versionId,
          );
      ref.invalidate(instancesProvider);
      if (!mounted) return;
      Navigator.pop(context);
      ScaffoldMessenger.of(context)
          .showSnackBar(SnackBar(content: Text('${project.title} ✓')));
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    return Scaffold(
      appBar: AppBar(title: Text(l10n.modsSearch)),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
            child: TextField(
              controller: _query,
              textInputAction: TextInputAction.search,
              decoration: InputDecoration(
                labelText: l10n.modsSearchPlaceholder,
                prefixIcon: const Icon(Icons.search_rounded),
                suffixIcon: IconButton(
                  icon: const Icon(Icons.arrow_forward_rounded),
                  onPressed: _search,
                ),
              ),
              onSubmitted: (_) => _search(),
            ),
          ),
          if (_error != null)
            Padding(
              padding: const EdgeInsets.all(12),
              child: Text(_error!, style: TextStyle(color: Theme.of(context).colorScheme.error)),
            ),
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : (_result == null
                    ? Center(child: Text(l10n.modsSearchPlaceholder))
                    : ListView.builder(
                        padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
                        itemCount: _result!.hits.length,
                        itemBuilder: (context, i) {
                          final p = _result!.hits[i];
                          return Card.outlined(
                            margin: const EdgeInsets.only(bottom: 10),
                            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(18)),
                            child: ListTile(
                              leading: ClipRRect(
                                borderRadius: BorderRadius.circular(8),
                                child: SizedBox(
                                  width: 40, height: 40,
                                  child: p.iconUrl != null
                                      ? Image.network(p.iconUrl!, errorBuilder: (_, _, _) => const Icon(Icons.extension_rounded))
                                      : const Icon(Icons.extension_rounded),
                                ),
                              ),
                              title: Text(p.title, maxLines: 1, overflow: TextOverflow.ellipsis, style: const TextStyle(fontWeight: FontWeight.w600)),
                              subtitle: Text(p.description, maxLines: 2, overflow: TextOverflow.ellipsis),
                              trailing: Text(formatNumber(p.downloads.toInt())),
                              onTap: () => _install(p),
                            ),
                          );
                        },
                      )),
          ),
        ],
      ),
    );
  }
}