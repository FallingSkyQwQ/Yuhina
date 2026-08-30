// Login bottom sheet: three tabs (Microsoft / Yggdrasil / Offline) plus the
// list of existing accounts with activate / remove actions.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../core/format.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import 'microsoft_login_flow.dart';
import 'yggdrasil_form.dart';

Future<void> showLoginSheet(BuildContext context) {
  return showModalBottomSheet<void>(
    context: context,
    isScrollControlled: true,
    builder: (_) => const FractionallySizedBox(
      heightFactor: 0.85,
      child: LoginSheet(),
    ),
  );
}

class LoginSheet extends ConsumerStatefulWidget {
  const LoginSheet({super.key});

  @override
  ConsumerState<LoginSheet> createState() => _LoginSheetState();
}

class _LoginSheetState extends ConsumerState<LoginSheet> {
  int _tab = 0;
  final _offlineName = TextEditingController();
  bool _offlineBusy = false;

  @override
  void dispose() {
    _offlineName.dispose();
    super.dispose();
  }

  Future<void> _addOffline() async {
    final l10n = AppLocalizations.of(context);
    final name = _offlineName.text.trim();
    if (name.isEmpty) return;
    setState(() => _offlineBusy = true);
    try {
      await ref.read(serviceProvider).addOfflineAccount(username: name);
      ref.invalidate(accountsProvider);
      ref.invalidate(activeAccountProvider);
      if (!mounted) return;
      Navigator.pop(context);
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context)
          .showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    } finally {
      if (mounted) setState(() => _offlineBusy = false);
    }
  }

  Future<void> _setActive(String id) async {
    final l10n = AppLocalizations.of(context);
    try {
      await ref.read(serviceProvider).setActiveAccount(id: id);
      ref.invalidate(accountsProvider);
      ref.invalidate(activeAccountProvider);
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context)
          .showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final accounts = ref.watch(accountsProvider).valueOrNull ?? const [];

    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.fromLTRB(20, 8, 20, 0),
          child: Text(l10n.settingsLogin, style: Theme.of(context).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w700)),
        ),
        SegmentedButton<int>(
          segments: [
            ButtonSegment(value: 0, label: Text(l10n.settingsMicrosoftLogin)),
            ButtonSegment(value: 1, label: Text(l10n.settingsYggdrasilLogin)),
            ButtonSegment(value: 2, label: Text(l10n.settingsOfflineLogin)),
          ],
          selected: {_tab},
          onSelectionChanged: (s) => setState(() => _tab = s.first),
        ),
        Expanded(
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(16),
            child: switch (_tab) {
              0 => const MicrosoftLoginFlow(),
              1 => const YggdrasilForm(),
              _ => _offlineForm(l10n),
            },
          ),
        ),
        const Divider(height: 1),
        Padding(
          padding: const EdgeInsets.all(12),
          child: accounts.isEmpty
              ? Text(l10n.homeNoAccount, style: Theme.of(context).textTheme.bodySmall)
              : Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    for (final acc in accounts) _accountRow(l10n, acc),
                  ],
                ),
        ),
      ],
    );
  }

  Widget _offlineForm(AppLocalizations l10n) {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          TextFormField(
            controller: _offlineName,
            decoration: InputDecoration(labelText: l10n.authOfflineName, prefixIcon: const Icon(Icons.person_rounded)),
          ),
          const SizedBox(height: 8),
          Text(l10n.authOfflineHint, style: Theme.of(context).textTheme.bodySmall),
          const SizedBox(height: 16),
          FilledButton(
            onPressed: _offlineBusy ? null : _addOffline,
            child: _offlineBusy
                ? const SizedBox(height: 20, width: 20, child: CircularProgressIndicator(strokeWidth: 2))
                : Text(l10n.authLoginButton),
          ),
        ],
      ),
    );
  }

  Widget _accountRow(AppLocalizations l10n, Account acc) {
    final scheme = Theme.of(context).colorScheme;
    return ListTile(
      dense: true,
      leading: acc.isActive
          ? Icon(Icons.check_circle_rounded, color: scheme.primary)
          : Icon(Icons.circle_outlined, color: scheme.outline),
      title: Text(acc.username),
      subtitle: Text(
        '${acc.kind.name} · ${formatDateTime(acc.expiresAt ?? 0)}',
        style: Theme.of(context).textTheme.bodySmall,
      ),
      trailing: acc.isActive
          ? null
          : TextButton(onPressed: () => _setActive(acc.id), child: Text(l10n.settingsActive)),
    );
  }
}