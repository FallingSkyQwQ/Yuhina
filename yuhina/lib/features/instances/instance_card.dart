// Instance card: icon, name, version/loader, mod count, size, last launch and
// a context menu (right-click on desktop / long-press / ⋮ button).

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:go_router/go_router.dart';

import '../../core/format.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';
import '../../theme/m3_expressive.dart';

class InstanceCard extends StatelessWidget {
  const InstanceCard({
    super.key,
    required this.instance,
    required this.onPlay,
    required this.onDelete,
    required this.onClone,
  });

  final InstanceSummary instance;
  final VoidCallback onPlay;
  final VoidCallback onDelete;
  final VoidCallback onClone;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final scheme = Theme.of(context).colorScheme;

    final card = tonalCard(
      context: context,
      onTap: () => context.push('/instances/${instance.id}'),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                CircleAvatar(
                  radius: 22,
                  backgroundColor: scheme.primaryContainer,
                  child: Text(instance.icon, style: const TextStyle(fontSize: 22)),
                ),
                const Spacer(),
                _cardMenu(context),
              ],
            ),
            const SizedBox(height: 12),
            Text(instance.name,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                style: Theme.of(context).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700)),
            const SizedBox(height: 2),
            Text(
              instance.loader != null
                  ? '${instance.mcVersion} · ${loaderTag(instance.loader!.kind)} ${instance.loader!.version}'
                  : instance.mcVersion,
              style: Theme.of(context).textTheme.bodySmall,
            ),
            const Spacer(),
            const SizedBox(height: 12),
            Wrap(
              spacing: 6,
              runSpacing: 6,
              children: [
                _chip(context, icon: Icons.extension_rounded, label: l10n.instancesModCount(instance.modCount)),
                _chip(context, icon: Icons.storage_rounded, label: formatBytes(instance.totalSizeBytes)),
              ],
            ),
            const SizedBox(height: 10),
            Row(
              children: [
                Expanded(
                  child: Text(
                    instance.lastLaunchedAt != null
                        ? '${l10n.instancesLastLaunched}: ${formatRelativeTime(instance.lastLaunchedAt!, justNow: 'now')}'
                        : l10n.instancesNeverLaunched,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                ),
                FilledButton.icon(
                  style: FilledButton.styleFrom(visualDensity: VisualDensity.compact),
                  onPressed: onPlay,
                  icon: const Icon(Icons.play_arrow_rounded, size: 18),
                  label: Text(l10n.instancesPlay),
                ),
              ],
            ),
          ],
        ),
      ),
    );

    return GestureDetector(
      onSecondaryTapDown: (details) => showMenu(
        context: context,
        position: RelativeRect.fromLTRB(details.globalPosition.dx, details.globalPosition.dy, 0, 0),
        items: _menuItems(context),
      ),
      child: card,
    );
  }

  List<PopupMenuEntry<String>> _menuItems(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    return [
      PopupMenuItem(value: 'clone', child: Text(l10n.instancesClone)),
      PopupMenuItem(value: 'delete', child: Text(l10n.instancesDelete)),
    ];
  }

  Widget _cardMenu(BuildContext context) {
    return PopupMenuButton<String>(
      icon: const Icon(Icons.more_vert_rounded),
      tooltip: 'menu',
      onSelected: (v) => switch (v) {
        'clone' => onClone(),
        'delete' => onDelete(),
        _ => null,
      },
      itemBuilder: (_) => _menuItems(context),
    );
  }

  static String loaderTag(LoaderKind kind) => switch (kind) {
        LoaderKind.forge => 'Forge',
        LoaderKind.fabric => 'Fabric',
        LoaderKind.neoForge => 'NeoForge',
        LoaderKind.quilt => 'Quilt',
      };

  Widget _chip(BuildContext context, {required IconData icon, required String label}) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 14, color: Theme.of(context).colorScheme.onSurfaceVariant),
          const SizedBox(width: 4),
          Text(label, style: Theme.of(context).textTheme.labelSmall),
        ],
      ),
    );
  }
}