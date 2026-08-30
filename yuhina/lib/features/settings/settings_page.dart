// Settings page: account / mirrors / java / general / about tabs.

import 'package:flutter/material.dart';
import 'package:yuhina/l10n/app_localizations.dart';

import 'about_tab.dart';
import 'accounts_tab.dart';
import 'general_tab.dart';
import 'java_tab.dart';
import 'mirrors_tab.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key, this.initialTab});

  final String? initialTab;

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  late int _index = switch (widget.initialTab) {
    'accounts' => 0,
    'mirrors' => 1,
    'java' => 2,
    'general' => 3,
    'about' => 4,
    _ => 3,
  };

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final titles = [
      l10n.settingsAccounts,
      l10n.settingsMirrors,
      l10n.settingsJava,
      l10n.settingsGeneral,
      l10n.settingsAbout,
    ];
    final icons = const [
      Icons.account_circle_rounded,
      Icons.cloud_sync_rounded,
      Icons.coffee_rounded,
      Icons.tune_rounded,
      Icons.info_rounded,
    ];
    final bodies = const [
      AccountsTab(),
      MirrorsTab(),
      JavaTab(),
      GeneralTab(),
      AboutTab(),
    ];

    return Scaffold(
      appBar: AppBar(title: Text(l10n.settingsTitle)),
      body: SafeArea(
        child: LayoutBuilder(
          builder: (context, constraints) {
            final wide = constraints.maxWidth >= 720;
            if (wide) {
              return Row(
                children: [
                  SizedBox(
                    width: 240,
                    child: ListView(
                      padding: const EdgeInsets.all(12),
                      children: [
                        for (var i = 0; i < titles.length; i++)
                          Padding(
                            padding: const EdgeInsets.only(bottom: 6),
                            child: _tabTile(i, icons[i], titles[i]),
                          ),
                      ],
                    ),
                  ),
                  const VerticalDivider(width: 1),
                  Expanded(child: bodies[_index]),
                ],
              );
            }
            return Column(
              children: [
                SegmentedButton<int>(
                  segments: [
                    for (var i = 0; i < titles.length; i++)
                      ButtonSegment(value: i, label: Text(titles[i]), icon: Icon(icons[i], size: 18)),
                  ],
                  selected: {_index},
                  onSelectionChanged: (s) => setState(() => _index = s.first),
                ),
                Expanded(child: bodies[_index]),
              ],
            );
          },
        ),
      ),
    );
  }

  Widget _tabTile(int i, IconData icon, String title) {
    final selected = i == _index;
    final scheme = Theme.of(context).colorScheme;
    return Material(
      color: selected ? scheme.secondaryContainer : Colors.transparent,
      borderRadius: BorderRadius.circular(14),
      child: InkWell(
        onTap: () => setState(() => _index = i),
        borderRadius: BorderRadius.circular(14),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
          child: Row(
            children: [
              Icon(icon, size: 20, color: selected ? scheme.onSecondaryContainer : scheme.onSurfaceVariant),
              const SizedBox(width: 12),
              Text(title, style: TextStyle(fontWeight: FontWeight.w600)),
            ],
          ),
        ),
      ),
    );
  }
}