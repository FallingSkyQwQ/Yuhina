// Root widget: MaterialApp + theme modes + locale + event-bus wiring.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import 'core/di.dart';
import 'core/event_bus.dart';
import 'router/app_router.dart';
import 'theme/app_theme.dart';

/// Wires the event bus once. Watch it from the root to keep it alive.
final _eventBusProvider = Provider<void>((ref) {
  wireEventBus(ref);
});

enum ThemeModeChoice {
  system(ThemeMode.system, 'settingsThemeModeSystem'),
  light(ThemeMode.light, 'settingsThemeModeLight'),
  dark(ThemeMode.dark, 'settingsThemeModeDark');

  final ThemeMode mode;
  final String l10nKey;
  const ThemeModeChoice(this.mode, this.l10nKey);

  static ThemeModeChoice fromMode(ThemeMode mode) =>
      values.firstWhere((e) => e.mode == mode, orElse: () => system);
}

final themeModeProvider = StateProvider<ThemeModeChoice>((ref) => ThemeModeChoice.system);

class YuhinaApp extends ConsumerWidget {
  const YuhinaApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // Keep the event bus alive for the whole app lifetime.
    ref.watch(_eventBusProvider);

    final themeMode = ref.watch(themeModeProvider).mode;
    final config = ref.watch(configProvider).valueOrNull;
    final seed = config?.themeSeed ?? 0;
    final locale = config?.locale ?? 'zh-CN';

    return MaterialApp.router(
      title: 'Yuhina',
      debugShowCheckedModeBanner: false,
      themeMode: themeMode,
      theme: buildAppTheme(seed: seed, brightness: Brightness.light),
      darkTheme: buildAppTheme(seed: seed, brightness: Brightness.dark),
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      locale: locale.startsWith('zh') ? const Locale('zh') : const Locale('en'),
      routerConfig: appRouter,
    );
  }
}

/// Shell for the three top-level tabs: expressive header + NavigationBar with
/// the M3 pill indicator.
class AppShell extends ConsumerWidget {
  const AppShell({super.key, required this.location, required this.child});

  final String location;
  final Widget child;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final int index = switch (location) {
      '/instances' => 1,
      '/downloads' => 2,
      _ => 0,
    };
    final titles = [l10n.navHome, l10n.navInstances, l10n.navDownloads];

    return Scaffold(
      body: SafeArea(
        child: Column(
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(24, 12, 12, 0),
              child: Row(
                children: [
                  Expanded(
                    child: Text(
                      titles[index],
                      style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                            fontWeight: FontWeight.w800,
                          ),
                    ),
                  ),
                  IconButton(
                    tooltip: l10n.logs,
                    icon: const Icon(Icons.terminal_rounded),
                    onPressed: () => context.go('/logs/latest'),
                  ),
                  IconButton(
                    tooltip: l10n.settings,
                    icon: const Icon(Icons.settings_rounded),
                    onPressed: () => context.go('/settings'),
                  ),
                ],
              ),
            ),
            Expanded(child: child),
          ],
        ),
      ),
      bottomNavigationBar: NavigationBar(
        selectedIndex: index,
        onDestinationSelected: (i) => switch (i) {
          0 => context.go('/'),
          1 => context.go('/instances'),
          _ => context.go('/downloads'),
        },
        destinations: [
          NavigationDestination(icon: const Icon(Icons.home_rounded), label: l10n.navHome),
          NavigationDestination(
              icon: const Icon(Icons.grid_view_rounded), label: l10n.navInstances),
          NavigationDestination(
              icon: const Icon(Icons.download_rounded), label: l10n.navDownloads),
        ],
      ),
    );
  }
}