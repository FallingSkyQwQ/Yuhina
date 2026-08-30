// go_router configuration. The three top-level destinations (Home / Instances
// / Downloads) live in a shell with a NavigationBar; detail pages push on top.

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../features/downloads/downloads_page.dart';
import '../features/home/home_page.dart';
import '../features/instances/instance_detail_page.dart';
import '../features/instances/instances_page.dart';
import '../features/logs/logs_page.dart';
import '../features/mods/mods_page.dart';
import '../app.dart';
import '../features/settings/settings_page.dart';
import '../theme/m3_expressive.dart';

final _rootNavigatorKey = GlobalKey<NavigatorState>();

final appRouter = GoRouter(
  navigatorKey: _rootNavigatorKey,
  initialLocation: '/',
  routes: [
    ShellRoute(
      builder: (_, state, child) => AppShell(
        location: state.matchedLocation,
        child: child,
      ),
      routes: [
        GoRoute(
          path: '/',
          builder: (_, _) => const HomePage(),
        ),
        GoRoute(
          path: '/instances',
          builder: (_, _) => const InstancesPage(),
        ),
        GoRoute(
          path: '/downloads',
          builder: (_, _) => const DownloadsPage(),
        ),
      ],
    ),
    GoRoute(
      parentNavigatorKey: _rootNavigatorKey,
      path: '/instances/:id',
      pageBuilder: (_, state) => expressivePageRoute(
        InstanceDetailPage(instanceId: state.pathParameters['id']!),
      ),
    ),
    GoRoute(
      parentNavigatorKey: _rootNavigatorKey,
      path: '/instances/:id/mods',
      pageBuilder: (_, state) => expressivePageRoute(
        ModsPage(instanceId: state.pathParameters['id']!),
      ),
    ),
    GoRoute(
      parentNavigatorKey: _rootNavigatorKey,
      path: '/settings',
      pageBuilder: (_, state) => expressivePageRoute(
        SettingsPage(initialTab: state.uri.queryParameters['tab']),
      ),
    ),
    GoRoute(
      parentNavigatorKey: _rootNavigatorKey,
      path: '/logs/:sessionId',
      pageBuilder: (_, state) =>
          expressivePageRoute(LogsPage(sessionId: state.pathParameters['sessionId']!)),
    ),
  ],
);