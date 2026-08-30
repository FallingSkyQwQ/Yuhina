// Mod tile: enable switch, name/file, update red-dot, tap for details.

import 'package:flutter/material.dart';

import '../../core/format.dart';
import '../../src/rust/third_party/yuhina_api/types.dart';

class ModTile extends StatelessWidget {
  const ModTile({
    super.key,
    required this.mod,
    required this.hasUpdate,
    required this.onToggle,
    required this.onTap,
  });

  final InstalledMod mod;
  final bool hasUpdate;
  final ValueChanged<bool> onToggle;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;

    return Material(
      color: scheme.surfaceContainerHigh,
      borderRadius: BorderRadius.circular(18),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(18),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
          child: Row(
            children: [
              Stack(
                clipBehavior: Clip.none,
                children: [
                  CircleAvatar(
                    radius: 20,
                    backgroundColor: scheme.primaryContainer,
                    child: Icon(Icons.extension_rounded, color: scheme.onPrimaryContainer),
                  ),
                  if (hasUpdate)
                    Positioned(
                      right: -2,
                      top: -2,
                      child: Container(
                        width: 12,
                        height: 12,
                        decoration: BoxDecoration(color: scheme.error, shape: BoxShape.circle, border: Border.all(color: scheme.surface, width: 2)),
                      ),
                    ),
                ],
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(mod.name.isEmpty ? mod.fileName : mod.name,
                        maxLines: 1, overflow: TextOverflow.ellipsis,
                        style: const TextStyle(fontWeight: FontWeight.w600)),
                    Text(
                      '${mod.fileName} · ${formatBytes(mod.fileSize.toInt())}',
                      maxLines: 1, overflow: TextOverflow.ellipsis,
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                  ],
                ),
              ),
              if (hasUpdate) Padding(
                padding: const EdgeInsets.only(right: 4),
                child: Icon(Icons.new_releases_rounded, size: 18, color: scheme.error),
              ),
              Switch(value: mod.enabled, onChanged: onToggle),
            ],
          ),
        ),
      ),
    );
  }
}