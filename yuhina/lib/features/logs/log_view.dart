// Log line list: monospaced, color-coded by level, optional filtering.

import 'package:flutter/material.dart';

import '../../src/rust/third_party/yuhina_api/types.dart';

class LogView extends StatelessWidget {
  const LogView({
    super.key,
    required this.entries,
    this.filter,
    this.emptyText = '—',
  });

  final List<GameLogEntry> entries;
  final Set<LogLevel>? filter;
  final String emptyText;

  @override
  Widget build(BuildContext context) {
    final visible = filter == null
        ? entries
        : entries.where((e) => filter!.contains(e.level)).toList();
    if (visible.isEmpty) {
      return Center(
        child: Text(emptyText, style: TextStyle(color: Theme.of(context).colorScheme.outline)),
      );
    }
    final scheme = Theme.of(context).colorScheme;
    return ListView.builder(
      padding: const EdgeInsets.all(12),
      itemCount: visible.length,
      itemBuilder: (context, i) {
        final e = visible[i];
        final color = switch (e.level) {
          LogLevel.error => scheme.error,
          LogLevel.warn => scheme.tertiary,
          _ => scheme.onSurfaceVariant,
        };
        return SelectableText(
          '[${_levelTag(e.level)}] ${e.text}',
          style: TextStyle(fontFamily: 'monospace', fontSize: 12, color: color),
        );
      },
    );
  }

  String _levelTag(LogLevel l) => switch (l) {
        LogLevel.info => 'INFO ',
        LogLevel.warn => 'WARN ',
        LogLevel.error => 'ERROR',
        LogLevel.debug => 'DEBUG',
      };
}