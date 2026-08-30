// The single YuhinaService instance handed over from main().
//
// `main()` calls `YuhinaService.newInstance` (which initializes RustLib's
// runtime + the Rust service layer) and overrides this provider so every page
// reaches the same FFI facade. Tests override it with a fake/mock.

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api.dart';
import '../src/rust/service.dart';
import '../src/rust/third_party/yuhina_api/types.dart';

final serviceProvider = Provider<YuhinaService>(
  (ref) => throw UnimplementedError(
    'serviceProvider must be overridden in main() (or a test override).',
  ),
);

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