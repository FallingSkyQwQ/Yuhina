import 'dart:ui';

import 'package:flutter_test/flutter_test.dart';
import 'package:yuhina/core/error_localizer.dart';
import 'package:yuhina/l10n/app_localizations.dart';
import 'package:yuhina/src/rust/api.dart';
import 'package:yuhina/src/rust/third_party/yuhina_api/error.dart';

void main() {
  final en = lookupAppLocalizations(const Locale('en'));

  test('maps every error kind', () {
    const kinds = <YuhinaErrorKind>[
      YuhinaErrorKind_Network(),
      YuhinaErrorKind_Http(404, 'https://x'),
      YuhinaErrorKind_Auth(),
      YuhinaErrorKind_AuthExpired(),
      YuhinaErrorKind_NotLoggedIn(),
      YuhinaErrorKind_VersionNotFound(),
      YuhinaErrorKind_LoaderNotInstalled(),
      YuhinaErrorKind_JavaNotFound(),
      YuhinaErrorKind_InvalidInstance(),
      YuhinaErrorKind_ModConflict(),
      YuhinaErrorKind_ModpackInvalid(),
      YuhinaErrorKind_ChecksumMismatch(),
      YuhinaErrorKind_DownloadFailed(),
      YuhinaErrorKind_Canceled(),
      YuhinaErrorKind_Io(),
      YuhinaErrorKind_Internal(),
    ];
    for (final k in kinds) {
      expect(localizeErrorKind(en, k), isNotEmpty);
    }
    expect(localizeErrorKind(en, const YuhinaErrorKind_Http(404, 'x')), 'HTTP error 404');
  });

  test('localizeError on YuhinaError includes message', () {
    final e = YuhinaError(kind: const YuhinaErrorKind_Auth(), message: 'bad token');
    expect(localizeError(en, e), contains('bad token'));
  });

  test('localizeError falls back to toString for unknown', () {
    expect(localizeError(en, StateError('boom')), 'Bad state: boom');
  });
}