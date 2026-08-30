// Event bus: watches `watchEvents()` and invalidates the matching providers so
// the UI always reflects the Rust service state.
//
// `watchProgress()` is additionally wired to refresh the download task list
// (Rust already throttles to 100ms per the contract).

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api.dart';
import 'di.dart';

/// Registers the listeners. Call once from the root widget (via ref).
void wireEventBus(Ref ref) {
  ref.listen(eventStreamProvider, (previous, next) {
    final event = next.value;
    if (event == null) return;
    switch (event) {
      case AppEvent_ConfigChanged():
        ref.invalidate(configProvider);
      case AppEvent_AccountsChanged():
        ref.invalidate(accountsProvider);
        ref.invalidate(activeAccountProvider);
      case AppEvent_InstancesChanged():
        ref.invalidate(instancesProvider);
        ref.invalidate(downloadTasksProvider);
      case AppEvent_TaskChanged():
        ref.invalidate(downloadTasksProvider);
      case AppEvent_JavaRuntimesChanged():
        ref.invalidate(javaRuntimesProvider);
      case AppEvent_VersionListChanged():
        ref.invalidate(versionListProvider);
    }
  });

  ref.listen(progressStreamProvider, (previous, next) {
    // Any progress/state change refreshes the task list view.
    ref.invalidate(downloadTasksProvider);
  });
}