// Download task tile: progress, speed, state, pause/resume/cancel.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';

import '../../core/format.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

class TaskTile extends StatelessWidget {
  const TaskTile({super.key, required this.task, required this.onAction});

  final DownloadTask task;
  final void Function(DownloadTask) onAction;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final scheme = Theme.of(context).colorScheme;
    final progress = task.totalBytes > BigInt.zero
        ? task.doneBytes.toInt() / task.totalBytes.toInt()
        : 0.0;

    final IconData icon = switch (task.state) {
      DownloadState.queued => Icons.hourglass_top_rounded,
      DownloadState.running => Icons.download_rounded,
      DownloadState.paused => Icons.pause_circle_outline_rounded,
      DownloadState.done => Icons.check_circle_outline_rounded,
      DownloadState.failed => Icons.error_outline_rounded,
      DownloadState.canceled => Icons.cancel_outlined,
    };

    return Card.outlined(
      margin: const EdgeInsets.only(bottom: 10),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(18)),
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(icon, color: _stateColor(scheme, task.state)),
                const SizedBox(width: 10),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(task.title,
                          maxLines: 1, overflow: TextOverflow.ellipsis,
                          style: const TextStyle(fontWeight: FontWeight.w600)),
                      Text(
                        _stateLabel(l10n, task.state),
                        style: Theme.of(context).textTheme.bodySmall,
                      ),
                    ],
                  ),
                ),
                if (task.state == DownloadState.running)
                  Text(formatSpeed(task.speedBps.toInt()),
                      style: Theme.of(context).textTheme.bodySmall),
              ],
            ),
            const SizedBox(height: 8),
            ClipRRect(
              borderRadius: BorderRadius.circular(6),
              child: LinearProgressIndicator(
                value: progress.clamp(0, 1),
                minHeight: 8,
                backgroundColor: scheme.surfaceContainerHighest,
              ),
            ),
            const SizedBox(height: 6),
            Row(
              children: [
                Expanded(
                  child: Text(
                    '${formatBytes(task.doneBytes.toInt())} / ${formatBytes(task.totalBytes.toInt())}',
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                ),
                if (task.canPause)
                  IconButton(
                    tooltip: l10n.downloadsPause,
                    icon: const Icon(Icons.pause_rounded),
                    onPressed: () => onAction(task),
                  ),
                if (task.state == DownloadState.paused ||
                    task.state == DownloadState.failed)
                  IconButton(
                    tooltip: l10n.downloadsResume,
                    icon: const Icon(Icons.play_arrow_rounded),
                    onPressed: () => onAction(task),
                  ),
                if (task.canCancel)
                  IconButton(
                    tooltip: l10n.downloadsCancel,
                    icon: const Icon(Icons.close_rounded),
                    onPressed: () => onAction(task),
                  ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Color _stateColor(ColorScheme scheme, DownloadState state) => switch (state) {
        DownloadState.running || DownloadState.queued => scheme.primary,
        DownloadState.done => scheme.primary,
        DownloadState.paused => scheme.tertiary,
        DownloadState.failed => scheme.error,
        DownloadState.canceled => scheme.outline,
      };

  String _stateLabel(AppLocalizations l10n, DownloadState state) => switch (state) {
        DownloadState.queued => l10n.downloadsStateQueued,
        DownloadState.running => l10n.downloadsStateRunning,
        DownloadState.paused => l10n.downloadsStatePaused,
        DownloadState.done => l10n.downloadsStateDone,
        DownloadState.failed => l10n.downloadsStateFailed,
        DownloadState.canceled => l10n.downloadsStateCanceled,
      };
}