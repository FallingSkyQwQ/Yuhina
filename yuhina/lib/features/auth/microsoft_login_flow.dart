// Microsoft OAuth flow: browser opens, we poll until the loopback callback
// completes, then surface the resulting account.

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

class MicrosoftLoginFlow extends ConsumerStatefulWidget {
  const MicrosoftLoginFlow({super.key});

  @override
  ConsumerState<MicrosoftLoginFlow> createState() => _MicrosoftLoginFlowState();
}

class _MicrosoftLoginFlowState extends ConsumerState<MicrosoftLoginFlow> {
  MicrosoftLoginHandle? _handle;
  Account? _completed;
  String? _error;
  bool _polling = false;
  Timer? _timer;

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  Future<void> _start() async {
    final l10n = AppLocalizations.of(context);
    setState(() {
      _error = null;
      _completed = null;
      _polling = true;
    });
    try {
      final handle = await ref.read(serviceProvider).beginMicrosoftLogin();
      if (!mounted) return;
      setState(() => _handle = handle);
      _timer = Timer.periodic(const Duration(seconds: 2), (_) => _poll());
    } on Object catch (e) {
      if (!mounted) return;
      setState(() {
        _polling = false;
        _error = localizeError(l10n, e);
      });
    }
  }

  Future<void> _poll() async {
    final handle = _handle;
    if (handle == null) return;
    try {
      final account = await ref.read(serviceProvider).pollMicrosoftLogin(handle: handle);
      if (!mounted) return;
      if (account != null) {
        _timer?.cancel();
        ref.invalidate(accountsProvider);
        ref.invalidate(activeAccountProvider);
        setState(() {
          _completed = account;
          _polling = false;
        });
      }
    } on Object catch (e) {
      _timer?.cancel();
      if (!mounted) return;
      setState(() {
        _polling = false;
        _error = localizeError(AppLocalizations.of(context), e);
      });
    }
  }

  Future<void> _cancel() async {
    final handle = _handle;
    _timer?.cancel();
    if (handle != null) {
      await ref.read(serviceProvider).cancelMicrosoftLogin(handle: handle);
    }
    if (!mounted) return;
    setState(() {
      _handle = null;
      _polling = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final scheme = Theme.of(context).colorScheme;

    if (_completed != null) {
      return Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.check_circle_rounded, size: 48, color: scheme.primary),
            const SizedBox(height: 12),
            Text(l10n.authLoginSuccess(_completed!.username),
                textAlign: TextAlign.center, style: const TextStyle(fontWeight: FontWeight.w600)),
            const SizedBox(height: 16),
            FilledButton(onPressed: () => Navigator.pop(context), child: Text(l10n.commonClose)),
          ],
        ),
      );
    }

    if (_handle == null) {
      return Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.language_rounded, size: 44, color: scheme.primary),
            const SizedBox(height: 12),
            Text(l10n.authMicrosoftHint, textAlign: TextAlign.center),
            if (_error != null) ...[
              const SizedBox(height: 8),
              Text(_error!, style: TextStyle(color: scheme.error)),
            ],
            const SizedBox(height: 16),
            FilledButton.icon(
              onPressed: _polling ? null : _start,
              icon: const Icon(Icons.login_rounded),
              label: Text(l10n.settingsMicrosoftLogin),
            ),
          ],
        ),
      );
    }

    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          const CircularProgressIndicator(),
          const SizedBox(height: 12),
          Text(l10n.authMicrosoftWaiting),
          const SizedBox(height: 16),
          OutlinedButton(onPressed: _cancel, child: Text(l10n.authMicrosoftCancel)),
        ],
      ),
    );
  }
}