// Instance edit sheet: rename + icon (the contract exposes only these two
// mutators, so launch-args/notes are shown read-only from the detail page).

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/di.dart';
import '../../core/error_localizer.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

Future<void> showEditInstanceSheet(
  BuildContext context,
  WidgetRef ref,
  InstanceDetail detail,
) {
  return showModalBottomSheet<void>(
    context: context,
    isScrollControlled: true,
    builder: (_) => EditInstanceSheet(ref: ref, detail: detail),
  );
}

class EditInstanceSheet extends ConsumerStatefulWidget {
  const EditInstanceSheet({super.key, required this.ref, required this.detail});

  final WidgetRef ref;
  final InstanceDetail detail;

  @override
  ConsumerState<EditInstanceSheet> createState() => _EditInstanceSheetState();
}

class _EditInstanceSheetState extends ConsumerState<EditInstanceSheet> {
  late final TextEditingController _name;
  late final TextEditingController _icon;
  late final TextEditingController _notes;
  bool _busy = false;

  @override
  void initState() {
    super.initState();
    _name = TextEditingController(text: widget.detail.summary.name);
    _icon = TextEditingController(text: widget.detail.summary.icon);
    _notes = TextEditingController(text: widget.detail.notes);
  }

  @override
  void dispose() {
    _name.dispose();
    _icon.dispose();
    _notes.dispose();
    super.dispose();
  }

  Future<void> _save() async {
    final l10n = AppLocalizations.of(context);
    setState(() => _busy = true);
    try {
      final svc = widget.ref.read(serviceProvider);
      await svc.renameInstance(id: widget.detail.summary.id, name: _name.text.trim());
      await svc.setInstanceIcon(id: widget.detail.summary.id, icon: _icon.text.trim());
      widget.ref.invalidate(instancesProvider);
      if (!mounted) return;
      Navigator.pop(context);
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(localizeError(l10n, e))));
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    return Padding(
      padding: EdgeInsets.only(bottom: MediaQuery.of(context).viewInsets.bottom),
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(l10n.instancesEdit, style: Theme.of(context).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w700)),
            const SizedBox(height: 16),
            TextField(
              controller: _name,
              decoration: InputDecoration(labelText: l10n.instanceNameLabel, prefixIcon: const Icon(Icons.badge_rounded)),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _icon,
              decoration: InputDecoration(labelText: l10n.instanceIconLabel, prefixIcon: const Icon(Icons.emoji_emotions_rounded)),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _notes,
              maxLines: 3,
              decoration: InputDecoration(labelText: l10n.instanceNotes, prefixIcon: const Icon(Icons.notes_rounded)),
            ),
            const SizedBox(height: 16),
            FilledButton(
              onPressed: _busy ? null : _save,
              child: _busy
                  ? const SizedBox(height: 20, width: 20, child: CircularProgressIndicator(strokeWidth: 2))
                  : Text(l10n.commonSave),
            ),
          ],
        ),
      ),
    );
  }
}