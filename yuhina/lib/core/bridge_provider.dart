// The single YuhinaService instance for the whole app.
//
// `startupProvider` boots RustLib + the Rust service layer; `serviceProvider`
// derives its value from it. The UI gates on `startupProvider` (splash / error)
// before any page reads the service, so `requireValue` is only ever observed in
// the ready state. Tests override `serviceProvider` with a fake/mock.

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api.dart';
import '../src/rust/frb_generated.dart';
import '../src/rust/service.dart';
import '../src/rust/third_party/yuhina_api/types.dart';

/// Boots RustLib + the YuhinaService. Runs once per app launch; `ref.invalidate`
/// re-runs it from the error screen.
final startupProvider = FutureProvider<YuhinaService>((ref) async {
  await RustLib.init();
  final service = await YuhinaService.newInstance(config: defaultLauncherConfig())
      .timeout(const Duration(seconds: 25));
  return service;
});

final serviceProvider = Provider<YuhinaService>((ref) {
  return ref.watch(startupProvider).requireValue;
});

/// Default boot configuration. data_dir/game_root use `~/` which the Rust
/// CorePaths expands on first use.
LauncherConfig defaultLauncherConfig() => LauncherConfig(
      dataDir: '~/.yuhina',
      gameRoot: '~/.yuhina/games',
      downloadSource: const Source.official(),
      customSourceHost: null,
      launchArgs: const LaunchArgs(
        minMemoryMb: 2048,
        maxMemoryMb: 4096,
        extraJvmArgs: [],
        extraMcArgs: [],
        windowWidth: null,
        windowHeight: null,
      ),
      locale: 'zh-CN',
      themeSeed: 0,
      autoUpdate: true,
    );