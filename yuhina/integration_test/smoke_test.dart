// Smoke test (M4/M5 gate, run on Linux CI with xvfb): verifies the FFI bridge
// boots end-to-end — service init, config round-trip, data paths and event
// stream connectivity. It intentionally does NOT launch a game.

import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:yuhina/src/rust/api.dart';
import 'package:yuhina/src/rust/frb_generated.dart';
import 'package:yuhina/src/rust/service.dart';
import 'package:yuhina/src/rust/third_party/yuhina_api/types.dart';

Future<void> main() async {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('smoke: service init + config roundtrip + events', (tester) async {
    final dylib = File('rust/target/release/libyuhina_bridge.so');
    if (dylib.existsSync()) {
      await RustLib.init(externalLibrary: ExternalLibrary.open(dylib.path));
    } else {
      await RustLib.init();
    }

    final tmp = Directory.systemTemp.createTempSync('yuhina-smoke');
    final config = LauncherConfig(
      dataDir: '${tmp.path}/data',
      gameRoot: '${tmp.path}/games',
      downloadSource: const Source.official(),
      customSourceHost: null,
      launchArgs: const LaunchArgs(
        minMemoryMb: 1024,
        maxMemoryMb: 2048,
        extraJvmArgs: [],
        extraMcArgs: [],
        windowWidth: null,
        windowHeight: null,
      ),
      locale: 'zh-CN',
      themeSeed: 0,
      autoUpdate: false,
    );

    // 1) Service init (aggregates core/download/instance/auth over yuhina.db).
    final service = await YuhinaService.newInstance(config: config);

    // 2) Config round-trip.
    final got = await service.getConfig();
    expect(got.dataDir, config.dataDir);
    expect(got.launchArgs.maxMemoryMb, 2048);

    // 3) Event stream connectivity: setConfig must emit ConfigChanged.
    final events = service.watchEvents();
    final first = events.first.timeout(const Duration(seconds: 10));
    await service.setConfig(config: config);
    final event = await first;
    expect(event, isA<AppEvent_ConfigChanged>());

    // 4) Data paths resolve to absolute directories.
    final (data, game) = await service.resolveDataPaths();
    expect(data, isNotEmpty);
    expect(game, isNotEmpty);
    expect(data, endsWith('data'));

    // 5) Progress stream is wired (empty, but a live stream).
    final progress = service.watchProgress();
    expect(progress, isA<Stream<DownloadProgressEvent>>());

    // 6) Domain services respond with empty initial state.
    expect(await service.listInstances(), isEmpty);
    expect(await service.listAccounts(), isEmpty);
    expect(await service.getActiveAccount().then((_) => true).catchError((_) => false), false);

    tmp.deleteSync(recursive: true);
  });
}