// Domain providers: each wraps one FFI method and is invalidated by the event
// bus (see event_bus.dart). Streaming sources are StreamProviders fed by the
// FRB streams.

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api.dart';
import '../src/rust/third_party/yuhina_api/error.dart';
import '../src/rust/third_party/yuhina_api/types.dart';
import 'bridge_provider.dart';

export 'bridge_provider.dart';

final configProvider = FutureProvider<LauncherConfig>((ref) async {
  return ref.watch(serviceProvider).getConfig();
});

final accountsProvider = FutureProvider<List<Account>>((ref) async {
  return ref.watch(serviceProvider).listAccounts();
});

final activeAccountProvider = FutureProvider<Account?>((ref) async {
  final svc = ref.watch(serviceProvider);
  try {
    return await svc.getActiveAccount();
  } on YuhinaError catch (e) {
    if (e.kind is YuhinaErrorKind_NotLoggedIn) return null;
    rethrow;
  }
});

final instancesProvider = FutureProvider<List<InstanceSummary>>((ref) async {
  return ref.watch(serviceProvider).listInstances();
});

/// Cached version list (empty until `fetchVersionList` is called).
final versionListProvider = FutureProvider<List<VersionMeta>>((ref) async {
  return ref.watch(serviceProvider).getVersionList();
});

final javaRuntimesProvider = FutureProvider<List<JavaRuntime>>((ref) async {
  return ref.watch(serviceProvider).listJavaRuntimes();
});

final newsProvider = FutureProvider<List<NewsItem>>((ref) async {
  return ref.watch(serviceProvider).getNews();
});

/// Download tasks, refreshed by `progressStreamProvider`.
final downloadTasksProvider = FutureProvider<List<DownloadTask>>((ref) async {
  final tasks = await ref.watch(serviceProvider).listDownloadTasks();
  // Listen so the provider stays alive & refreshes on progress events.
  ref.watch(progressStreamProvider);
  return tasks;
});

final gameSessionsProvider = FutureProvider<List<GameSession>>((ref) async {
  return ref.watch(serviceProvider).listGameSessions();
});

// ---------------------------------------------------------------------------
// Streams
// ---------------------------------------------------------------------------

final eventStreamProvider = StreamProvider<AppEvent>((ref) {
  return ref.watch(serviceProvider).watchEvents();
});

final progressStreamProvider = StreamProvider<DownloadProgressEvent>((ref) {
  return ref.watch(serviceProvider).watchProgress();
});

/// Real-time output for one game session (lives for the session lifetime).
StreamProvider<GameOutput> gameOutputProvider(String sessionId) {
  return StreamProvider<GameOutput>((ref) {
    return ref.watch(serviceProvider).watchGameOutput(sessionId: sessionId);
  });
}

/// Accumulates the per-event output stream into a growing list for display.
StreamProvider<List<GameOutput>> gameOutputListProvider(String sessionId) {
  return StreamProvider<List<GameOutput>>((ref) async* {
    final acc = <GameOutput>[];
    await for (final e in ref.watch(serviceProvider).watchGameOutput(sessionId: sessionId)) {
      acc.add(e);
      yield List.of(acc);
    }
  });
}