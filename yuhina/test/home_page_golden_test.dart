import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yuhina/features/home/home_page.dart';
import 'package:yuhina/src/rust/third_party/yuhina_api/types.dart';

import 'helpers/fake_service.dart';
import 'helpers/pump.dart';

void main() {
  Future<void> pumpHome(WidgetTester tester, {Brightness brightness = Brightness.light}) async {
    await tester.binding.setSurfaceSize(const Size(900, 700));
    addTearDown(() => tester.binding.setSurfaceSize(null));
    final service = FakeYuhinaService(
      accounts: [sampleAccount()],
      instances: [sampleInstance()],
      news: const [
        NewsItem(title: 'Snapshot 25w02a', url: 'https://example.com', published: '2026-08-01', summary: 'Exciting new snapshot with the copper golem.'),
        NewsItem(title: 'New DLC', url: 'https://example.com/2', published: '2026-07-20', summary: 'Dungeons DLC released.'),
      ],
    );
    await pumpApp(tester, const HomePage(), service);
  }

  testWidgets('home page golden (light)', (tester) async {
    await pumpHome(tester);
    await expectLater(find.byType(HomePage), matchesGoldenFile('goldens/home_light.png'));
  });

  testWidgets('home page golden (dark)', (tester) async {
    await pumpHome(tester, brightness: Brightness.dark);
    await expectLater(find.byType(HomePage), matchesGoldenFile('goldens/home_dark.png'));
  });

  testWidgets('home page shows quick start + account + news', (tester) async {
    await pumpHome(tester);
    expect(find.text('Test Instance'), findsOneWidget);
    expect(find.text('Steve'), findsOneWidget);
    expect(find.text('Snapshot 25w02a'), findsOneWidget);
  });
}