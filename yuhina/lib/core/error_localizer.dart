// Maps FFI `YuhinaError` kinds to localized, human-readable text.

import 'package:yuhina/l10n/app_localizations.dart';

import '../src/rust/api.dart';
import '../src/rust/third_party/yuhina_api/error.dart';

/// Localized message for a thrown `YuhinaError` (FRB throws this on error
/// results). Falls back to the raw `message` when the kind is unknown.
String localizeError(AppLocalizations l10n, Object error) {
  if (error is YuhinaError) {
    return '${localizeErrorKind(l10n, error.kind)}: ${error.message}';
  }
  return error.toString();
}

String localizeErrorKind(AppLocalizations l10n, YuhinaErrorKind kind) {
  switch (kind) {
    case YuhinaErrorKind_Network():
      return l10n.errorNetwork;
    case YuhinaErrorKind_Http(:final field0):
      return l10n.errorHttp(field0);
    case YuhinaErrorKind_Auth():
      return l10n.errorAuth;
    case YuhinaErrorKind_AuthExpired():
      return l10n.errorAuthExpired;
    case YuhinaErrorKind_NotLoggedIn():
      return l10n.errorNotLoggedIn;
    case YuhinaErrorKind_VersionNotFound():
      return l10n.errorVersionNotFound;
    case YuhinaErrorKind_LoaderNotInstalled():
      return l10n.errorLoaderNotInstalled;
    case YuhinaErrorKind_JavaNotFound():
      return l10n.errorJavaNotFound;
    case YuhinaErrorKind_InvalidInstance():
      return l10n.errorInvalidInstance;
    case YuhinaErrorKind_ModConflict():
      return l10n.errorModConflict;
    case YuhinaErrorKind_ModpackInvalid():
      return l10n.errorModpackInvalid;
    case YuhinaErrorKind_ChecksumMismatch():
      return l10n.errorChecksumMismatch;
    case YuhinaErrorKind_DownloadFailed():
      return l10n.errorDownloadFailed;
    case YuhinaErrorKind_Canceled():
      return l10n.errorCanceled;
    case YuhinaErrorKind_Io():
      return l10n.errorIo;
    case YuhinaErrorKind_Internal():
      return l10n.errorInternal;
  }
}