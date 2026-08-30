// Logs page: live game output, level filter, crash summary.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../src/rust/api.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import 'log_view.dart';

class LogsPage extends ConsumerStatefulWidget {
  const LogsPage({super.key, required this.sessionId});

  final String sessionId;

  @override
  ConsumerState<LogsPage> createState() => _LogsPageState();
}

class _LogsPageState extends ConsumerState<LogsPage> {
  Set<LogLevel> _filter = {LogLevel.info, LogLevel.warn, LogLevel.error};
  String? _resolvedSessionId;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _resolveSession());
  }

  Future<void> _resolveSession() async {
    if (widget.sessionId != 'latest') {
      setState(() => _resolvedSessionId = widget.sessionId);
      return;
    }
    final sessions = await ref.read(serviceProvider).listGameSessions();
    if (!mounted) return;
    if (sessions.isNotEmpty) {
      sessions.sort((a, b) => b.startedAt.compareTo(a.startedAt));
      setState(() => _resolvedSessionId = sessions.first.sessionId);
    } else {
      setState(() => _resolvedSessionId = '');
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final sessionId = _resolvedSessionId;

    return Scaffold(
      appBar: AppBar(title: Text(l10n.logsTitle)),
      body: sessionId == null
          ? const Center(child: CircularProgressIndicator())
          : sessionId.isEmpty
              ? Center(child: Text(l10n.logsEmpty))
              : _SessionLogs(sessionId: sessionId, l10n: l10n, filter: _filter),
      bottomNavigationBar: sessionId == null || sessionId.isEmpty
          ? null
          : _levelFilter(l10n),
    );
  }

  Widget _levelFilter(AppLocalizations l10n) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 4, 16, 4),
        child: Wrap(
          spacing: 6,
          children: [
            for (final level in [LogLevel.info, LogLevel.warn, LogLevel.error, LogLevel.debug])
              FilterChip(
                label: Text(_levelLabel(l10n, level)),
                selected: _filter.contains(level),
                onSelected: (sel) => setState(() {
                  if (sel) {
                    _filter.add(level);
                  } else {
                    _filter.remove(level);
                  }
                  _filter = Set.of(_filter);
                }),
              ),
          ],
        ),
      ),
    );
  }

  String _levelLabel(AppLocalizations l10n, LogLevel l) => switch (l) {
        LogLevel.info => l10n.logsLevelInfo,
        LogLevel.warn => l10n.logsLevelWarn,
        LogLevel.error => l10n.logsLevelError,
        LogLevel.debug => l10n.logsLevelDebug,
      };
}

class _SessionLogs extends ConsumerWidget {
  const _SessionLogs({required this.sessionId, required this.l10n, required this.filter});

  final String sessionId;
  final AppLocalizations l10n;
  final Set<LogLevel> filter;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final session = ref.watch(_sessionProvider(sessionId)).valueOrNull;
    final live = ref.watch(gameOutputListProvider(sessionId));
    final history = ref.watch(_historyProvider(sessionId)).valueOrNull ?? const [];

    final liveLines = [
      for (final out in live.value ?? const <GameOutput>[])
        GameLogEntry(
          index: BigInt.zero,
          level: out.level,
          text: out.text,
          ts: BigInt.zero,
        ),
    ];
    final entries = [...history, ...liveLines];

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (session != null)
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 8, 16, 0),
            child: _sessionHeader(context, session),
          ),
        Expanded(
          child: LogView(entries: entries, filter: filter, emptyText: l10n.logsEmpty),
        ),
      ],
    );
  }

  Widget _sessionHeader(BuildContext context, GameSession session) {
    final scheme = Theme.of(context).colorScheme;
    final (color, label) = switch (session.state) {
      GameState_Starting() || GameState_Running() => (scheme.primary, l10n.logsStateRunning),
      GameState_Stopped() => (scheme.primary, l10n.logsStateStopped),
      GameState_Crashed(:final field0) => (scheme.error, '${l10n.logsStateCrashed}: $field0'),
    };
    return Wrap(
      spacing: 8,
      crossAxisAlignment: WrapCrossAlignment.center,
      children: [
        Icon(Icons.circle, size: 10, color: color),
        Text('PID ${session.pid}', style: Theme.of(context).textTheme.bodySmall),
        Text(label, style: Theme.of(context).textTheme.bodySmall),
      ],
    );
  }
}

final _sessionProvider = FutureProvider.family<GameSession?, String>((ref, id) async {
  try {
    return await ref.watch(serviceProvider).getGameSession(sessionId: id);
  } on Object {
    return null; // ended sessions are replayed from the log file
  }
});

final _historyProvider = FutureProvider.family<List<GameLogEntry>, String>((ref, id) async {
  return ref.watch(serviceProvider).getGameLogs(sessionId: id, afterIndex: BigInt.zero);
});