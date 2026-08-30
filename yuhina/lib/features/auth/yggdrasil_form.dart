// Yggdrasil login form: presets (LittleSkin) + custom server URL.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';

const String kLittleSkinUrl = 'https://littleskin.cn/api/yggdrasil';

class YggdrasilForm extends ConsumerStatefulWidget {
  const YggdrasilForm({super.key});

  @override
  ConsumerState<YggdrasilForm> createState() => _YggdrasilFormState();
}

class _YggdrasilFormState extends ConsumerState<YggdrasilForm> {
  final _formKey = GlobalKey<FormState>();
  final _server = TextEditingController(text: kLittleSkinUrl);
  final _username = TextEditingController();
  final _password = TextEditingController();
  bool _busy = false;
  String? _error;

  @override
  void dispose() {
    _server.dispose();
    _username.dispose();
    _password.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    final l10n = AppLocalizations.of(context);
    if (!_formKey.currentState!.validate()) return;
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      await ref.read(serviceProvider).addYggdrasilAccount(
            serverUrl: _server.text.trim(),
            username: _username.text.trim(),
            password: _password.text,
          );
      ref.invalidate(accountsProvider);
      ref.invalidate(activeAccountProvider);
      if (!mounted) return;
      Navigator.pop(context);
      ScaffoldMessenger.of(context)
          .showSnackBar(SnackBar(content: Text(l10n.authLoginSuccess(_username.text.trim()))));
    } on Object catch (e) {
      if (!mounted) return;
      setState(() {
        _busy = false;
        _error = localizeError(l10n, e);
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final scheme = Theme.of(context).colorScheme;

    return Form(
      key: _formKey,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(l10n.authYggdrasilPreset, style: Theme.of(context).textTheme.labelLarge),
            const SizedBox(height: 8),
            Wrap(
              spacing: 8,
              children: [
                ActionChip(
                  avatar: const Icon(Icons.person_pin_rounded, size: 18),
                  label: Text(l10n.authYggdrasilLittleSkin),
                  onPressed: () => setState(() => _server.text = kLittleSkinUrl),
                ),
              ],
            ),
            const SizedBox(height: 16),
            TextFormField(
              controller: _server,
              decoration: InputDecoration(labelText: l10n.authYggdrasilServer, prefixIcon: const Icon(Icons.dns_rounded)),
              validator: (v) => (v == null || v.trim().isEmpty) ? l10n.authYggdrasilServer : null,
            ),
            const SizedBox(height: 12),
            TextFormField(
              controller: _username,
              decoration: InputDecoration(labelText: l10n.authOfflineName, prefixIcon: const Icon(Icons.person_rounded)),
              validator: (v) => (v == null || v.trim().isEmpty) ? l10n.authOfflineName : null,
            ),
            const SizedBox(height: 12),
            TextFormField(
              controller: _password,
              obscureText: true,
              decoration: InputDecoration(labelText: l10n.errorAuth, prefixIcon: const Icon(Icons.lock_rounded)),
              validator: (v) => (v == null || v.isEmpty) ? l10n.errorAuth : null,
            ),
            if (_error != null) ...[
              const SizedBox(height: 8),
              Text(_error!, style: TextStyle(color: scheme.error)),
            ],
            const SizedBox(height: 16),
            FilledButton(
              onPressed: _busy ? null : _submit,
              child: _busy
                  ? const SizedBox(height: 20, width: 20, child: CircularProgressIndicator(strokeWidth: 2))
                  : Text(l10n.authLoginButton),
            ),
          ],
        ),
      ),
    );
  }
}