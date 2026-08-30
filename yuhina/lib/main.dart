// App entrypoint.
//
// The window is shown immediately with a branded splash so the user never
// stares at an empty frame while the Rust service layer boots. The service is
// constructed in the background (with a hard timeout); on failure the splash
// turns into an error screen with a retry action.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'app.dart';
import 'core/bridge_provider.dart';
import 'src/rust/frb_generated.dart';
import 'src/rust/service.dart';

/// Boots RustLib + the YuhinaService. Runs once per app launch; `ref.invalidate`
/// re-runs it from the error screen.
final startupProvider = FutureProvider<YuhinaService>((ref) async {
  await RustLib.init();
  final service = await YuhinaService.newInstance(config: defaultLauncherConfig())
      .timeout(const Duration(seconds: 25));
  return service;
});

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(
    const ProviderScope(child: StartupGate()),
  );
}

/// Decides between splash / error / real app based on service boot state.
class StartupGate extends ConsumerWidget {
  const StartupGate({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final boot = ref.watch(startupProvider);

    return boot.when(
      loading: () => const BootSplash(),
      error: (e, st) => BootError(message: '$e'),
      data: (service) => ProviderScope(
        overrides: [serviceProvider.overrideWithValue(service)],
        child: const YuhinaApp(),
      ),
    );
  }
}

/// Branded loading screen shown while the service layer initializes.
class BootSplash extends StatelessWidget {
  const BootSplash({super.key});

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return Material(
      color: scheme.surface,
      child: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              width: 96,
              height: 96,
              decoration: BoxDecoration(
                color: scheme.primaryContainer,
                borderRadius: BorderRadius.circular(28),
              ),
              child: Icon(Icons.games, size: 56, color: scheme.onPrimaryContainer),
            ),
            const SizedBox(height: 24),
            Text('Yuhina', style: Theme.of(context).textTheme.headlineMedium),
            const SizedBox(height: 16),
            const SizedBox(
              width: 28,
              height: 28,
              child: CircularProgressIndicator(strokeWidth: 3),
            ),
          ],
        ),
      ),
    );
  }
}

/// Shown when the service failed to boot; offers a retry.
class BootError extends ConsumerWidget {
  const BootError({super.key, required this.message});

  final String message;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final scheme = Theme.of(context).colorScheme;
    return Material(
      color: scheme.surface,
      child: Center(
        child: Padding(
          padding: const EdgeInsets.all(32),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(Icons.error_outline, size: 64, color: scheme.error),
              const SizedBox(height: 16),
              Text('启动失败 / Failed to start', style: Theme.of(context).textTheme.titleLarge),
              const SizedBox(height: 12),
              ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 480),
                child: Text(
                  message,
                  textAlign: TextAlign.center,
                  maxLines: 6,
                  overflow: TextOverflow.ellipsis,
                  style: Theme.of(context).textTheme.bodySmall,
                ),
              ),
              const SizedBox(height: 24),
              FilledButton.icon(
                onPressed: () => ref.invalidate(startupProvider),
                icon: const Icon(Icons.refresh),
                label: const Text('重试 / Retry'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}