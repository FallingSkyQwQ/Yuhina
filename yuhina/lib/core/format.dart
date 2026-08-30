// Human-friendly formatting helpers shared across pages.
//
// The FFI maps Rust `u64` to Dart `BigInt`; helpers here accept `Object` and
// normalize to `int` (safe: these are sizes/timestamps within int64 range).

import 'package:intl/intl.dart';

final NumberFormat _nf = NumberFormat.decimalPattern();

int _toInt(Object v) => v is BigInt ? v.toInt() : (v as num).toInt();

/// "1.2 GB", "340 KB" …
String formatBytes(Object bytes) {
  final b = _toInt(bytes);
  if (b <= 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  var v = b.toDouble();
  var u = 0;
  while (v >= 1024 && u < units.length - 1) {
    v /= 1024;
    u++;
  }
  final s = u == 0 ? v.toStringAsFixed(0) : v.toStringAsFixed(1);
  return '$s ${units[u]}';
}

/// "1.2 MB/s"
String formatSpeed(Object bytesPerSecond) => '${formatBytes(bytesPerSecond)}/s';

/// Millis → "2026-08-30 04:15" (local time).
String formatDateTime(Object millis) {
  final m = _toInt(millis);
  if (m <= 0) return '—';
  final dt = DateTime.fromMillisecondsSinceEpoch(m);
  return DateFormat('yyyy-MM-dd HH:mm').format(dt);
}

/// Relative time: "just now", "3m ago", "2h ago", "3d ago".
String formatRelativeTime(Object millis, {required String justNow}) {
  final m = _toInt(millis);
  if (m <= 0) return '—';
  final diff = DateTime.now().difference(DateTime.fromMillisecondsSinceEpoch(m));
  if (diff.inSeconds < 60) return justNow;
  if (diff.inMinutes < 60) return '${diff.inMinutes}m';
  if (diff.inHours < 24) return '${diff.inHours}h';
  return '${diff.inDays}d';
}

String formatNumber(Object n) => _nf.format(_toInt(n));