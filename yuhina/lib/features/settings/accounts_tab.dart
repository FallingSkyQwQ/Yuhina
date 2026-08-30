// Accounts tab: sign in (opens the login sheet), activate / refresh / remove.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../core/format.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import '../auth/login_sheet.dart';

class AccountsTab extends ConsumerWidget {
  const AccountsTab({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final accounts = ref.watch(accountsProvider);

    return ListView(
      padding: const EdgeInsets.all(20),
      children: [
        Align(
          alignment: Alignment.centerRight,
          child: FilledButton.icon(
            onPressed: () => showLoginSheet(context),
            icon: const Icon(Icons.login_rounded),
            label: Text(l10n.settingsLogin),
          ),
        ),
        const SizedBox(height: 12),
        accounts.when(
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (e, _) => Text(localizeError(l10n, e)),
          data: (list) {
            if (list.isEmpty) return Text(l10n.homeNoAccount);
            return Column(
              children: [
                for (final a in list) _accountCard(context, ref, l10n, a),
              ],
            );
          },
        ),
      ],
    );
  }

  Widget _accountCard(BuildContext context, WidgetRef ref, AppLocalizations l10n, Account a) {
    final scheme = Theme.of(context).colorScheme;
    return Card.outlined(
      margin: const EdgeInsets.only(bottom: 10),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(18)),
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Row(
          children: [
            CircleAvatar(
              radius: 20,
              backgroundColor: a.isActive ? scheme.primaryContainer : scheme.surfaceContainerHighest,
              child: Icon(Icons.person_rounded, color: a.isActive ? scheme.onPrimaryContainer : scheme.onSurfaceVariant),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Text(a.username, style: const TextStyle(fontWeight: FontWeight.w700)),
                      if (a.isActive) ...[
                        const SizedBox(width: 6),
                        Icon(Icons.verified_rounded, size: 16, color: scheme.primary),
                      ],
                    ],
                  ),
                  Text(
                    '${_kindLabel(a.kind)} · ${formatDateTime(a.expiresAt ?? 0)}',
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                ],
              ),
            ),
            IconButton(
              tooltip: l10n.settingsRefreshAccount,
              icon: const Icon(Icons.refresh_rounded),
              onPressed: () => _refresh(context, ref, l10n, a.id),
            ),
            IconButton(
              tooltip: l10n.settingsLogout,
              icon: const Icon(Icons.logout_rounded),
              onPressed: () => _remove(context, ref, l10n, a),
            ),
          ],
        ),
      ),
    );
  }

  String _kindLabel(AccountKind k) => switch (k) {
        AccountKind.microsoft => 'Microsoft',
        AccountKind.yggdrasil => 'Yggdrasil',
        AccountKind.offline => 'Offline',
      };

  Future<void> _refresh(BuildContext context, WidgetRef ref, AppLocalizations l10n, String id) async {
    try {
      await ref.read(serviceProvider).refreshAccount(id: id);
      ref.invalidate(accountsProvider);
      ref.invalidate(activeAccountProvider);
    } on Object catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
      }
    }
  }

  Future<void> _remove(BuildContext context, WidgetRef ref, AppLocalizations l10n, Account a) async {
    try {
      await ref.read(serviceProvider).removeAccount(id: a.id);
      ref.invalidate(accountsProvider);
      ref.invalidate(activeAccountProvider);
    } on Object catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
      }
    }
  }
}