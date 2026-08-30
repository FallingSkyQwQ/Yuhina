import 'package:flutter_test/flutter_test.dart';
import 'package:yuhina/core/format.dart';

void main() {
  test('formatBytes handles units and BigInt', () {
    expect(formatBytes(0), '0 B');
    expect(formatBytes(512), '512 B');
    expect(formatBytes(2048), '2.0 KB');
    expect(formatBytes(BigInt.from(5 * 1024 * 1024)), '5.0 MB');
    expect(formatBytes(BigInt.from(3 * 1024 * 1024 * 1024)), '3.0 GB');
  });

  test('formatSpeed appends /s', () {
    expect(formatSpeed(1024 * 1024), '1.0 MB/s');
  });

  test('formatDateTime formats millis', () {
    // 2026-08-30 04:15 local
    final dt = DateTime(2026, 8, 30, 4, 15);
    expect(formatDateTime(dt.millisecondsSinceEpoch), '2026-08-30 04:15');
    expect(formatDateTime(0), '—');
  });

  test('formatRelativeTime', () {
    expect(formatRelativeTime(0, justNow: 'now'), '—');
    final now = DateTime.now().millisecondsSinceEpoch;
    expect(formatRelativeTime(now - 1000, justNow: 'now'), 'now');
    expect(formatRelativeTime(now - 1000 * 60 * 5, justNow: 'now'), '5m');
    expect(formatRelativeTime(now - 1000 * 60 * 60 * 3, justNow: 'now'), '3h');
  });

  test('formatNumber', () {
    expect(formatNumber(12345), '12,345');
    expect(formatNumber(BigInt.from(12345)), '12,345');
  });
}