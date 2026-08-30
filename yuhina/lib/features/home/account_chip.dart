// Active-account chip: shows the signed-in player, or a "sign in" affordance
// that opens the login sheet.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../auth/login_sheet.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

class AccountChip extends ConsumerWidget {
  const AccountChip({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final account = ref.watch(activeAccountProvider).valueOrNull;

    if (account == null) {
      return _AccountTile(
        icon: const Icon(Icons.person_off_rounded),
        title: l10n.homeNoAccount,
        subtitle: l10n.settingsLogin,
        onTap: () => showLoginSheet(context),
      );
    }
    return _AccountTile(
      icon: CircleAvatar(
        backgroundColor: Theme.of(context).colorScheme.primaryContainer,
        child: Text(
          account.username.isNotEmpty ? account.username[0].toUpperCase() : '?',
          style: TextStyle(fontWeight: FontWeight.w700, color: Theme.of(context).colorScheme.onPrimaryContainer),
        ),
      ),
      title: account.username,
      subtitle: _kindLabel(account.kind),
      onTap: () => showLoginSheet(context),
    );
  }

  String _kindLabel(AccountKind kind) => switch (kind) {
        AccountKind.microsoft => 'Microsoft',
        AccountKind.yggdrasil => 'Yggdrasil',
        AccountKind.offline => 'Offline',
      };
}

class _AccountTile extends StatelessWidget {
  const _AccountTile({
    required this.icon,
    required this.title,
    required this.subtitle,
    required this.onTap,
  });

  final Widget icon;
  final String title;
  final String subtitle;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Theme.of(context).colorScheme.surfaceContainerHigh,
      borderRadius: BorderRadius.circular(20),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(20),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          child: Row(
            children: [
              icon,
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(title, style: const TextStyle(fontWeight: FontWeight.w600)),
                    Text(subtitle, style: Theme.of(context).textTheme.bodySmall),
                  ],
                ),
              ),
              const Icon(Icons.chevron_right_rounded),
            ],
          ),
        ),
      ),
    );
  }
}