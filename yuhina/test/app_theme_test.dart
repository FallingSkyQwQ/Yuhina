import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yuhina/theme/app_theme.dart';

void main() {
  test('builds light and dark themes from a seed', () {
    final light = buildAppTheme(seed: 0x6C5CE7, brightness: Brightness.light);
    final dark = buildAppTheme(seed: 0x6C5CE7, brightness: Brightness.dark);

    expect(light.colorScheme.primary, isNotNull);
    expect(dark.colorScheme.primary, isNotNull);
    // Different seed → different primary.
    final other = buildAppTheme(seed: 0xFF0000, brightness: Brightness.light);
    expect(other.colorScheme.primary, isNot(light.colorScheme.primary));
    // Expressive radii applied.
    expect(light.dialogTheme.shape, isA<RoundedRectangleBorder>());
  });

  test('seed 0 falls back to the default seed', () {
    final def = buildAppTheme(seed: 0, brightness: Brightness.light);
    final named = buildAppTheme(seed: 0x6C5CE7, brightness: Brightness.light);
    expect(def.colorScheme.primary, named.colorScheme.primary);
  });
}