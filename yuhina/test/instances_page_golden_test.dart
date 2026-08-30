import 'dart:io' show Platform;

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yuhina/features/instances/instances_page.dart';
import 'package:yuhina/src/rust/third_party/yuhina_api/types.dart';

import 'helpers/fake_service.dart';
import 'helpers/pump.dart';

void main() {
  // Goldens are generated on Linux; font rasterization differs across
  // platforms, so pixel comparison only holds on the reference platform.
  if (Platform.isWindows || Platform.isMacOS) {
    return;
  }

  Future<void> pumpInstances(WidgetTester tester, {Brightness brightness = Brightness.light}) async {
    await tester.binding.setSurfaceSize(const Size(900, 700));
    addTearDown(() => tester.binding.setSurfaceSize(null));
    final service = FakeYuhinaService(
      instances: [
        sampleInstance(),
        sampleInstance(
          id: 'i2',
          name: 'Fabric 1.21',
          mcVersion: '1.21.1',
          loader: const Loader(kind: LoaderKind.fabric, version: '0.16.0'),
          lastLaunchedAt: DateTime(2026, 8, 29, 20).millisecondsSinceEpoch,
        ),
      ],
    );
    await pumpApp(tester, const InstancesPage(), service);
  }

  testWidgets('instances page golden (light)', (tester) async {
    await pumpInstances(tester);
    await expectLater(find.byType(InstancesPage), matchesGoldenFile('goldens/instances_light.png'));
  });

  testWidgets('instances page golden (dark)', (tester) async {
    await pumpInstances(tester, brightness: Brightness.dark);
    await expectLater(find.byType(InstancesPage), matchesGoldenFile('goldens/instances_dark.png'));
  });

  testWidgets('instances page lists cards', (tester) async {
    await pumpInstances(tester);
    expect(find.text('Test Instance'), findsOneWidget);
    expect(find.text('Fabric 1.21'), findsOneWidget);
    expect(find.textContaining('1.21.1'), findsWidgets);
  });
}