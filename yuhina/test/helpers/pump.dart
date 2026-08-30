// Widget-test harness: provides the fake service + a working router + l10n.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:yuhina/core/bridge_provider.dart';
import 'package:yuhina/router/app_router.dart';
import 'package:yuhina/theme/app_theme.dart';

import 'fake_service.dart';

Widget wrapService(
  Widget child,
  FakeYuhinaService service, {
  Brightness brightness = Brightness.light,
  bool useRouter = false,
}) {
  final app = MaterialApp(
    locale: const Locale('zh'),
    supportedLocales: AppLocalizations.supportedLocales,
    localizationsDelegates: AppLocalizations.localizationsDelegates,
    theme: buildAppTheme(seed: 0x6C5CE7, brightness: brightness),
    debugShowCheckedModeBanner: false,
    home: child,
  );
  return ProviderScope(
    overrides: [serviceProvider.overrideWithValue(service)],
    child: useRouter
        ? MaterialApp.router(
            routerConfig: appRouter,
            locale: const Locale('zh'),
            supportedLocales: AppLocalizations.supportedLocales,
            localizationsDelegates: AppLocalizations.localizationsDelegates,
            theme: buildAppTheme(seed: 0x6C5CE7, brightness: brightness),
            debugShowCheckedModeBanner: false,
          )
        : app,
  );
}

Future<void> pumpApp(
  WidgetTester tester,
  Widget child,
  FakeYuhinaService service,
) async {
  await tester.pumpWidget(wrapService(child, service));
  await tester.pump();
}