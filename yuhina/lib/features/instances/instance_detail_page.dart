// Instance detail: big play button, Java, launch args, loader install, Mods
// entry, and game logs entry.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../core/format.dart';
import '../../src/rust/api.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import '../../theme/m3_expressive.dart';

class InstanceDetailPage extends ConsumerWidget {
  const InstanceDetailPage({super.key, required this.instanceId});

  final String instanceId;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final detail = ref.watch(_detailProvider(instanceId));

    return Scaffold(
      appBar: AppBar(title: Text(l10n.instanceDetail)),
      body: detail.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (e, _) => Center(child: Text(localizeError(l10n, e))),
        data: (d) => _DetailBody(context: context, ref: ref, l10n: l10n, detail: d),
      ),
    );
  }
}

final _detailProvider = FutureProvider.family<InstanceDetail, String>((ref, id) {
  return ref.watch(serviceProvider).getInstance(id: id);
});

class _DetailBody extends StatelessWidget {
  const _DetailBody({required this.context, required this.ref, required this.l10n, required this.detail});

  final BuildContext context;
  final WidgetRef ref;
  final AppLocalizations l10n;
  final InstanceDetail detail;

  @override
  Widget build(BuildContext context) {
    final s = detail.summary;
    final scheme = Theme.of(context).colorScheme;

    return ListView(
      padding: const EdgeInsets.fromLTRB(24, 8, 24, 96),
      children: [
        tonalCard(
          context: context,
          padding: const EdgeInsets.all(24),
          child: Column(
            children: [
              CircleAvatar(
                radius: 40,
                backgroundColor: scheme.primaryContainer,
                child: Text(s.icon, style: const TextStyle(fontSize: 38)),
              ),
              const SizedBox(height: 12),
              Text(s.name,
                  style: Theme.of(context).textTheme.headlineSmall?.copyWith(fontWeight: FontWeight.w800)),
              const SizedBox(height: 4),
              Text(
                s.loader != null
                    ? '${s.mcVersion} · ${_loaderName(s.loader!.kind)} ${s.loader!.version}'
                    : s.mcVersion,
                style: Theme.of(context).textTheme.bodyMedium,
              ),
              const SizedBox(height: 4),
              Text(
                s.isInstalled ? l10n.instancesInstalled : l10n.instancesNotInstalled,
                style: Theme.of(context).textTheme.bodySmall?.copyWith(
                    color: s.isInstalled ? scheme.primary : scheme.error),
              ),
              const SizedBox(height: 20),
              SizedBox(
                width: double.infinity,
                child: FilledButton.icon(
                  style: FilledButton.styleFrom(minimumSize: const Size.fromHeight(52)),
                  onPressed: () => _launch(ref),
                  icon: const Icon(Icons.play_arrow_rounded, size: 28),
                  label: Text(l10n.instancesPlay),
                ),
              ),
              const SizedBox(height: 8),
              OutlinedButton.icon(
                onPressed: () => _installLoader(ref),
                icon: const Icon(Icons.build_rounded),
                label: Text(l10n.instancesInstallLoader),
              ),
            ],
          ),
        ),
        const SizedBox(height: 16),
        tonalCard(
          context: context,
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              _infoRow(l10n.instanceJavaLabel, _javaLabel(detail.java)),
              _infoRow(l10n.instancesMods, '${s.modCount}'),
              _infoRow(l10n.instancesSize, formatBytes(s.totalSizeBytes.toInt())),
              _infoRow(l10n.instancesLastLaunched, s.lastLaunchedAt != null
                  ? formatDateTime(s.lastLaunchedAt!.toInt())
                  : l10n.instancesNeverLaunched),
              _infoRow(l10n.instancesOpenDir, ''),
              if (detail.notes.isNotEmpty) _infoRow(l10n.instanceNotes, detail.notes),
            ],
          ),
        ),
        const SizedBox(height: 16),
        tonalCard(
          context: context,
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(l10n.instanceArgs, style: Theme.of(context).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700)),
              const SizedBox(height: 8),
              _infoRow(l10n.instanceMinMemory, '${(detail.launchArgs?.minMemoryMb ?? 2048)} MB'),
              _infoRow(l10n.instanceMaxMemory, '${(detail.launchArgs?.maxMemoryMb ?? 4096)} MB'),
              _infoRow(l10n.instanceExtraJvm, (detail.launchArgs?.extraJvmArgs ?? const []).join(' ')),
              _infoRow(l10n.instanceExtraMc, (detail.launchArgs?.extraMcArgs ?? const []).join(' ')),
            ],
          ),
        ),
        const SizedBox(height: 16),
        Row(
          children: [
            Expanded(
              child: FilledButton.tonalIcon(
                onPressed: () => context.push('/instances/$instanceId/mods'),
                icon: const Icon(Icons.extension_rounded),
                label: Text(l10n.modsTitle),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: FilledButton.tonalIcon(
                onPressed: () => context.push('/logs/latest'),
                icon: const Icon(Icons.terminal_rounded),
                label: Text(l10n.instanceLogs),
              ),
            ),
          ],
        ),
      ],
    );
  }

  String get instanceId => detail.summary.id;

  Future<void> _launch(WidgetRef ref) async {
    final messenger = ScaffoldMessenger.of(context);
    try {
      await ref.read(serviceProvider).launchInstance(instanceId: instanceId);
      messenger.showSnackBar(SnackBar(content: Text('${detail.summary.name} ▶')));
    } on Object catch (e) {
      messenger.showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    }
  }

  Future<void> _installLoader(WidgetRef ref) async {
    if (detail.summary.loader != null) {
      ScaffoldMessenger.of(context)
          .showSnackBar(SnackBar(content: Text(l10n.instancesInstalled)));
      return;
    }
    final loader = await _pickLoader(context, l10n, detail.summary.mcVersion);
    if (loader == null) return;
    try {
      await ref.read(serviceProvider).installInstanceLoader(id: instanceId, loader: loader);
      ref.invalidate(instancesProvider);
      ref.invalidate(_detailProvider(instanceId));
    } on Object catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
      }
    }
  }

  String _javaLabel(JavaSelection j) => switch (j) {
        JavaSelection_Auto(:final field0) =>
          l10n.instanceJavaAuto(field0),
        JavaSelection_Manual(:final field0) =>
          l10n.instanceJavaManual(field0),
      };

  Widget _infoRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(width: 140, child: Text(label, style: Theme.of(context).textTheme.bodyMedium?.copyWith(color: Theme.of(context).colorScheme.onSurfaceVariant))),
          Expanded(child: Text(value.isEmpty ? '—' : value)),
        ],
      ),
    );
  }

  static String _loaderName(LoaderKind k) => switch (k) {
        LoaderKind.forge => 'Forge',
        LoaderKind.fabric => 'Fabric',
        LoaderKind.neoForge => 'NeoForge',
        LoaderKind.quilt => 'Quilt',
      };

  Future<Loader?> _pickLoader(BuildContext context, AppLocalizations l10n, String mcVersion) async {
    final kind = await showDialog<LoaderKind>(
      context: context,
      builder: (ctx) => SimpleDialog(
        title: Text(l10n.instancesInstallLoader),
        children: [
          for (final k in LoaderKind.values)
            SimpleDialogOption(
              onPressed: () => Navigator.pop(ctx, k),
              child: Text(_loaderName(k)),
            ),
        ],
      ),
    );
    if (kind == null) return null;
    final version = await showDialog<String>(
      // ignore: use_build_context_synchronously
      context: context,
      builder: (ctx) {
        final c = TextEditingController();
        return AlertDialog(
          title: Text(l10n.modsVersion),
          content: TextField(controller: c, autofocus: true, decoration: const InputDecoration(hintText: '0.16.0')),
          actions: [
            TextButton(onPressed: () => Navigator.pop(ctx), child: Text(l10n.commonCancel)),
            // ignore: use_build_context_synchronously
            FilledButton(
              onPressed: () => Navigator.pop(ctx, c.text.trim()),
              child: Text(l10n.commonConfirm),
            ),
          ],
        );
      },
    );
    if (version == null || version.isEmpty) return null;
    return Loader(kind: kind, version: version);
  }
}