// App entrypoint: boot the FFI service, then start the UI.
//
// `YuhinaService.newInstance` constructs the whole Rust service layer over
// `~/.yuhina/yuhina.db` and wires the event streams, so the UI can render
// against the real backend immediately.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'app.dart';
import 'core/bridge_provider.dart';
import 'src/rust/frb_generated.dart';
import 'src/rust/service.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await RustLib.init();
  final service = await YuhinaService.newInstance(config: defaultLauncherConfig());

  runApp(
    ProviderScope(
      overrides: [serviceProvider.overrideWithValue(service)],
      child: const YuhinaApp(),
    ),
  );
}