import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yuhina/router/app_router.dart';

import 'helpers/fake_service.dart';
import 'helpers/pump.dart';

void main() {
  setUp(() => appRouter.go('/'));

  Future<void> pumpShell(WidgetTester tester) async {
    await pumpApp(tester, const SizedBox.shrink(), FakeYuhinaService());
    await tester.pumpWidget(wrapService(const SizedBox.shrink(), FakeYuhinaService(), useRouter: true));
    await tester.pump();
  }

  testWidgets('navigates to every route', (tester) async {
    await pumpShell(tester);

    for (final path in ['/', '/instances', '/downloads', '/instances/i1', '/instances/i1/mods', '/settings', '/logs/s1']) {
      appRouter.go(path);
      await tester.pumpAndSettle();
      expect(appRouter.state.matchedLocation, path, reason: 'for $path');
    }
  });

  testWidgets('shell renders NavigationBar on top-level routes', (tester) async {
    await pumpShell(tester);
    appRouter.go('/instances');
    await tester.pumpAndSettle();
    expect(find.byType(NavigationBar), findsOneWidget);
  });
}