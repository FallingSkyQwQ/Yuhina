// Home: quick launch + account chip + Mojang news.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'account_chip.dart';
import 'news_section.dart';
import 'quick_start.dart';

class HomePage extends ConsumerWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return ListView(
      padding: const EdgeInsets.fromLTRB(24, 16, 24, 96),
      children: const [
        QuickStart(),
        SizedBox(height: 16),
        AccountChip(),
        SizedBox(height: 24),
        NewsSection(),
      ],
    );
  }
}