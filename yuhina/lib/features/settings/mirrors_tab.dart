// Mirrors & sources tab: switch download source (Official / BMCLAPI / Custom).

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/api.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

class MirrorsTab extends ConsumerStatefulWidget {
  const MirrorsTab({super.key});

  @override
  ConsumerState<MirrorsTab> createState() => _MirrorsTabState();
}

class _MirrorsTabState extends ConsumerState<MirrorsTab> {
  late TextEditingController _customHost;
  Source? _source;
  bool _saving = false;

  @override
  void initState() {
    super.initState();
    _customHost = TextEditingController();
  }

  @override
  void dispose() {
    _customHost.dispose();
    super.dispose();
  }

  Future<void> _save(Source source, String? host) async {
    final l10n = AppLocalizations.of(context);
    final config = ref.read(configProvider).valueOrNull;
    if (config == null) return;
    setState(() => _saving = true);
    try {
      await ref.read(serviceProvider).setConfig(
            config: LauncherConfig(
              dataDir: config.dataDir,
              gameRoot: config.gameRoot,
              downloadSource: source,
              customSourceHost: host,
              launchArgs: config.launchArgs,
              locale: config.locale,
              themeSeed: config.themeSeed,
              autoUpdate: config.autoUpdate,
            ),
          );
      ref.invalidate(configProvider);
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    } finally {
      if (mounted) setState(() => _saving = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final config = ref.watch(configProvider).valueOrNull;
    _source = config?.downloadSource;
    if (config != null && _customHost.text.isEmpty && config.customSourceHost != null) {
      _customHost.text = config.customSourceHost!;
    }

    return ListView(
      padding: const EdgeInsets.all(20),
      children: [
        Text(l10n.settingsDownloadSource, style: Theme.of(context).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700)),
        const SizedBox(height: 12),
        Card.outlined(
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(18)),
          child: RadioGroup<Source>(
            groupValue: _source,
            onChanged: (s) {
              if (s == null) return;
              _save(s, s is Source_Custom ? _customHost.text.trim() : null);
            },
            child: Column(
              children: [
                RadioListTile<Source>(
                  value: const Source.official(),
                  title: Text(l10n.settingsSourceOfficial),
                ),
                RadioListTile<Source>(
                  value: const Source.bmclapi(),
                  title: Text(l10n.settingsSourceBmclapi),
                ),
                RadioListTile<Source>(
                  value: const Source.custom(''),
                  title: Text(l10n.settingsSourceCustom),
                ),
              ],
            ),
          ),
        ),
        if (_source case Source_Custom()) ...[
          const SizedBox(height: 12),
          TextField(
            controller: _customHost,
            decoration: InputDecoration(
              labelText: l10n.settingsCustomHost,
              prefixIcon: const Icon(Icons.link_rounded),
              suffixIcon: IconButton(
                icon: const Icon(Icons.check_rounded),
                onPressed: () => _save(_source!, _customHost.text.trim()),
              ),
            ),
          ),
        ],
        const SizedBox(height: 8),
        if (_saving) const LinearProgressIndicator(),
      ],
    );
  }
}