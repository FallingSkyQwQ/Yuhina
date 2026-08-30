// "New instance" bottom sheet: name, icon, MC version, loader, Java.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/api.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

Future<void> showCreateInstanceSheet(BuildContext context, WidgetRef ref) {
  return showModalBottomSheet<void>(
    context: context,
    isScrollControlled: true,
    builder: (_) => FractionallySizedBox(heightFactor: 0.9, child: CreateInstanceSheet(ref: ref)),
  );
}

class CreateInstanceSheet extends ConsumerStatefulWidget {
  const CreateInstanceSheet({super.key, required this.ref});

  final WidgetRef ref;

  @override
  ConsumerState<CreateInstanceSheet> createState() => _CreateInstanceSheetState();
}

class _CreateInstanceSheetState extends ConsumerState<CreateInstanceSheet> {
  final _formKey = GlobalKey<FormState>();
  final _name = TextEditingController(text: 'My Instance');
  final _icon = TextEditingController(text: '🎮');
  final _loaderVersion = TextEditingController();
  String? _mcVersion;
  LoaderKind? _loaderKind;
  JavaSelection _java = const JavaSelection.auto(21);
  bool _busy = false;
  String? _error;

  @override
  void dispose() {
    _name.dispose();
    _icon.dispose();
    _loaderVersion.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    final l10n = AppLocalizations.of(context);
    if (!_formKey.currentState!.validate()) return;
    final loader = _loaderKind == null
        ? null
        : Loader(kind: _loaderKind!, version: _loaderVersion.text.trim());
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      await widget.ref.read(serviceProvider).createInstance(
            req: CreateInstanceRequest(
              name: _name.text.trim(),
              icon: _icon.text.trim().isEmpty ? '🎮' : _icon.text.trim(),
              mcVersion: _mcVersion!,
              loader: loader,
              java: _java,
              dirName: null,
            ),
          );
      widget.ref.invalidate(instancesProvider);
      if (!mounted) return;
      Navigator.pop(context);
    } on Object catch (e) {
      if (!mounted) return;
      setState(() {
        _busy = false;
        _error = localizeError(l10n, e);
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final versions = widget.ref.watch(versionListProvider);
    final scheme = Theme.of(context).colorScheme;

    return Padding(
      padding: EdgeInsets.only(bottom: MediaQuery.of(context).viewInsets.bottom),
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Form(
          key: _formKey,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(l10n.instancesNew, style: Theme.of(context).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w700)),
              const SizedBox(height: 16),
              TextFormField(
                controller: _name,
                decoration: InputDecoration(labelText: l10n.instanceNameLabel, prefixIcon: const Icon(Icons.badge_rounded)),
                validator: (v) => (v == null || v.trim().isEmpty) ? l10n.instanceNameLabel : null,
              ),
              const SizedBox(height: 12),
              TextFormField(
                controller: _icon,
                decoration: InputDecoration(labelText: l10n.instanceIconLabel, prefixIcon: const Icon(Icons.emoji_emotions_rounded)),
              ),
              const SizedBox(height: 12),
              versions.when(
                loading: () => const LinearProgressIndicator(),
                error: (e, _) => Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(localizeError(l10n, e)),
                    TextButton.icon(
                      onPressed: () async {
                        try {
                          await widget.ref.read(serviceProvider).fetchVersionList();
                          widget.ref.invalidate(versionListProvider);
                        } catch (e2) {
                          if (context.mounted) {
                            ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e2))));
                          }
                        }
                      },
                      icon: const Icon(Icons.download_rounded, size: 18),
                      label: Text(l10n.homeFetchVersions),
                    ),
                  ],
                ),
                data: (list) => DropdownButtonFormField<String>(
                  initialValue: _mcVersion,
                  decoration: InputDecoration(labelText: l10n.instanceMcVersionLabel, prefixIcon: const Icon(Icons.extension_rounded)),
                  items: [
                    for (final v in list) DropdownMenuItem(value: v.id, child: Text(v.id)),
                  ],
                  onChanged: (v) => setState(() => _mcVersion = v),
                  validator: (v) => v == null ? l10n.instanceMcVersionLabel : null,
                ),
              ),
              const SizedBox(height: 12),
              DropdownButtonFormField<LoaderKind?>(
                initialValue: _loaderKind,
                decoration: InputDecoration(labelText: l10n.instanceLoaderLabel, prefixIcon: const Icon(Icons.extension_rounded)),
                items: [
                  DropdownMenuItem(value: null, child: Text(l10n.instanceLoaderNone)),
                  for (final k in LoaderKind.values)
                    DropdownMenuItem(value: k, child: Text(loaderName(k))),
                ],
                onChanged: (v) => setState(() => _loaderKind = v),
              ),
              if (_loaderKind != null) ...[
                const SizedBox(height: 12),
                TextFormField(
                  controller: _loaderVersion,
                  decoration: InputDecoration(
                    labelText: l10n.modsVersion,
                    hintText: '0.16.0',
                    prefixIcon: const Icon(Icons.tag_rounded),
                  ),
                  validator: (v) => (v == null || v.trim().isEmpty) ? l10n.modsVersion : null,
                ),
              ],
              const SizedBox(height: 12),
              _javaPicker(l10n),
              if (_error != null) ...[
                const SizedBox(height: 8),
                Text(_error!, style: TextStyle(color: scheme.error)),
              ],
              const SizedBox(height: 20),
              FilledButton(
                onPressed: _busy ? null : _submit,
                child: _busy
                    ? const SizedBox(height: 20, width: 20, child: CircularProgressIndicator(strokeWidth: 2))
                    : Text(l10n.instanceCreate),
              ),
            ],
          ),
        ),
      ),
    );
  }

  String loaderName(LoaderKind k) => switch (k) {
        LoaderKind.forge => 'Forge',
        LoaderKind.fabric => 'Fabric',
        LoaderKind.neoForge => 'NeoForge',
        LoaderKind.quilt => 'Quilt',
      };

  Widget _javaPicker(AppLocalizations l10n) {
    final majors = [8, 11, 17, 21];
    final auto = _java is JavaSelection_Auto;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(l10n.instanceJavaLabel, style: Theme.of(context).textTheme.labelLarge),
        const SizedBox(height: 8),
        SegmentedButton<bool>(
          segments: [
            ButtonSegment(value: true, label: Text(l10n.instanceJavaAuto('auto'))),
            ButtonSegment(value: false, label: Text(l10n.instanceJavaManual(''))),
          ],
          selected: {auto},
          onSelectionChanged: (s) {
            setState(() {
              _java = s.first
                  ? const JavaSelection.auto(21)
                  : const JavaSelection.manual('/usr/lib/jvm/.../bin/java');
            });
          },
        ),
        if (auto)
          DropdownButtonFormField<int>(
            initialValue: _java is JavaSelection_Auto ? (_java as JavaSelection_Auto).field0 : 21,
            decoration: const InputDecoration(labelText: 'Java major'),
            items: [for (final m in majors) DropdownMenuItem(value: m, child: Text('$m'))],
            onChanged: (v) => setState(() => _java = JavaSelection.auto(v ?? 21)),
          ),
      ],
    );
  }
}