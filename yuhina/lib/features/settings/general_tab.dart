// General tab: language, theme mode, theme seed, auto-update.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../app.dart';
import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

class GeneralTab extends ConsumerStatefulWidget {
  const GeneralTab({super.key});

  @override
  ConsumerState<GeneralTab> createState() => _GeneralTabState();
}

class _GeneralTabState extends ConsumerState<GeneralTab> {
  bool _saving = false;

  Future<void> _save(LauncherConfig Function(LauncherConfig) mutate) async {
    final l10n = AppLocalizations.of(context);
    final config = ref.read(configProvider).valueOrNull;
    if (config == null) return;
    final next = mutate(config);
    setState(() => _saving = true);
    try {
      await ref.read(serviceProvider).setConfig(config: next);
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
    final themeMode = ref.watch(themeModeProvider);

    return ListView(
      padding: const EdgeInsets.all(20),
      children: [
        _section(l10n.settingsLanguage, Row(
          children: [
            for (final loc in const ['zh-CN', 'en-US'])
              Padding(
                padding: const EdgeInsets.only(right: 8),
                child: ChoiceChip(
                  label: Text(loc),
                  selected: config?.locale == loc,
                  onSelected: (_) => _save((c) => LauncherConfig(
                        dataDir: c.dataDir,
                        gameRoot: c.gameRoot,
                        downloadSource: c.downloadSource,
                        customSourceHost: c.customSourceHost,
                        launchArgs: c.launchArgs,
                        locale: loc,
                        themeSeed: c.themeSeed,
                        autoUpdate: c.autoUpdate,
                      )),
                ),
              ),
          ],
        )),
        _section(l10n.settingsThemeMode, SegmentedButton<ThemeModeChoice>(
          segments: [
            for (final m in ThemeModeChoice.values)
              ButtonSegment(value: m, label: Text(_modeLabel(l10n, m))),
          ],
          selected: {themeMode},
          onSelectionChanged: (s) => ref.read(themeModeProvider.notifier).state = s.first,
        )),
        _section(l10n.settingsThemeSeed, _seedPicker(context, l10n, config)),
        SwitchListTile(
          title: Text(l10n.settingsAutoUpdate),
          value: config?.autoUpdate ?? true,
          onChanged: (v) => _save((c) => LauncherConfig(
                dataDir: c.dataDir,
                gameRoot: c.gameRoot,
                downloadSource: c.downloadSource,
                customSourceHost: c.customSourceHost,
                launchArgs: c.launchArgs,
                locale: c.locale,
                themeSeed: c.themeSeed,
                autoUpdate: v,
              )),
        ),
        if (_saving) const LinearProgressIndicator(),
      ],
    );
  }

  String _modeLabel(AppLocalizations l10n, ThemeModeChoice m) => switch (m) {
        ThemeModeChoice.system => l10n.settingsThemeModeSystem,
        ThemeModeChoice.light => l10n.settingsThemeModeLight,
        ThemeModeChoice.dark => l10n.settingsThemeModeDark,
      };

  Widget _seedPicker(BuildContext context, AppLocalizations l10n, LauncherConfig? config) {
    const seeds = [0x6C5CE7, 0x1E8E3E, 0xC62828, 0x1565C0, 0x6A1B9A, 0xEF6C00];
    return Wrap(
      spacing: 10,
      children: [
        for (final s in seeds)
          InkWell(
            onTap: () => _save((c) => LauncherConfig(
                  dataDir: c.dataDir,
                  gameRoot: c.gameRoot,
                  downloadSource: c.downloadSource,
                  customSourceHost: c.customSourceHost,
                  launchArgs: c.launchArgs,
                  locale: c.locale,
                  themeSeed: s,
                  autoUpdate: c.autoUpdate,
                )),
            borderRadius: BorderRadius.circular(24),
            child: Container(
              width: 40,
              height: 40,
              decoration: BoxDecoration(
                color: Color(s),
                shape: BoxShape.circle,
                border: config?.themeSeed == s
                    ? Border.all(color: Theme.of(context).colorScheme.onSurface, width: 3)
                    : null,
              ),
            ),
          ),
      ],
    );
  }

  Widget _section(String title, Widget child) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 20),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(title, style: Theme.of(context).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700)),
          const SizedBox(height: 8),
          child,
        ],
      ),
    );
  }
}