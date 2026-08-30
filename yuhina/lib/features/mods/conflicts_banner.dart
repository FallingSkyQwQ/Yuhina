// Conflict banner: lists mod conflicts detected by the Rust conflict checker.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';

import '../../src/rust/third_party/yuhina_api/types.dart';
import '../../theme/m3_expressive.dart';

class ConflictsBanner extends StatelessWidget {
  const ConflictsBanner({super.key, required this.conflicts});

  final List<ModConflict> conflicts;

  @override
  Widget build(BuildContext context) {
    if (conflicts.isEmpty) return const SizedBox.shrink();
    final l10n = AppLocalizations.of(context);
    final scheme = Theme.of(context).colorScheme;
    final hasError = conflicts.any((c) => c.severity == ConflictSeverity.error);

    return tonalCard(
      context: context,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(hasError ? Icons.error_rounded : Icons.warning_amber_rounded,
                    color: hasError ? scheme.error : scheme.tertiary),
                const SizedBox(width: 8),
                Text('${l10n.modsConflicts} (${conflicts.length})',
                    style: const TextStyle(fontWeight: FontWeight.w700)),
              ],
            ),
            const SizedBox(height: 8),
            for (final c in conflicts)
              Padding(
                padding: const EdgeInsets.only(bottom: 6),
                child: Text(
                  '• ${c.message}',
                  style: TextStyle(
                    color: c.severity == ConflictSeverity.error ? scheme.error : scheme.onSurfaceVariant,
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }
}