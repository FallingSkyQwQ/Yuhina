// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get appTitle => 'Yuhina Launcher';

  @override
  String get navHome => 'Home';

  @override
  String get navInstances => 'Instances';

  @override
  String get navDownloads => 'Downloads';

  @override
  String get settings => 'Settings';

  @override
  String get logs => 'Logs';

  @override
  String get commonCancel => 'Cancel';

  @override
  String get commonConfirm => 'Confirm';

  @override
  String get commonDelete => 'Delete';

  @override
  String get commonSave => 'Save';

  @override
  String get commonClose => 'Close';

  @override
  String get commonSearch => 'Search';

  @override
  String get commonRefresh => 'Refresh';

  @override
  String get commonLoading => 'Loading…';

  @override
  String get commonRetry => 'Retry';

  @override
  String get commonError => 'Error';

  @override
  String get commonEmpty => 'Nothing here yet';

  @override
  String get commonBack => 'Back';

  @override
  String get commonOk => 'OK';

  @override
  String get commonCopy => 'Copy';

  @override
  String get commonName => 'Name';

  @override
  String get commonOpen => 'Open';

  @override
  String get homeQuickLaunch => 'Quick Launch';

  @override
  String get homeNews => 'News';

  @override
  String get homeNewsUnavailable => 'News unavailable';

  @override
  String get homeActiveAccount => 'Active account';

  @override
  String get homeNoAccount => 'Not signed in';

  @override
  String get homeNoVersion => 'No cached version list';

  @override
  String get homeFetchVersions => 'Fetch versions';

  @override
  String get instancesTitle => 'Instances';

  @override
  String get instancesNew => 'New instance';

  @override
  String get instancesPlay => 'Play';

  @override
  String get instancesEdit => 'Edit';

  @override
  String get instancesClone => 'Clone';

  @override
  String get instancesRename => 'Rename';

  @override
  String get instancesDelete => 'Delete';

  @override
  String get instancesOpenDir => 'Open folder';

  @override
  String get instancesInstallLoader => 'Install loader';

  @override
  String get instancesMods => 'Mods';

  @override
  String get instancesEmpty => 'No instances yet. Create one to get started.';

  @override
  String get instancesNotInstalled => 'Not installed';

  @override
  String get instancesInstalled => 'Installed';

  @override
  String get instancesLastLaunched => 'Last launched';

  @override
  String get instancesNeverLaunched => 'Never launched';

  @override
  String instancesModCount(Object count) {
    return '$count mods';
  }

  @override
  String get instancesSize => 'Size';

  @override
  String get instanceNameLabel => 'Name';

  @override
  String get instanceIconLabel => 'Icon';

  @override
  String get instanceMcVersionLabel => 'Minecraft version';

  @override
  String get instanceLoaderLabel => 'Loader';

  @override
  String get instanceLoaderNone => 'Vanilla';

  @override
  String get instanceCreate => 'Create';

  @override
  String instanceDeleteConfirm(Object name) {
    return 'Delete instance “$name” permanently?';
  }

  @override
  String get instanceDeleteFiles => 'Also delete game files';

  @override
  String get instanceJavaLabel => 'Java';

  @override
  String instanceJavaAuto(Object major) {
    return 'Auto (major $major)';
  }

  @override
  String instanceJavaManual(Object path) {
    return 'Manual ($path)';
  }

  @override
  String get instanceNotes => 'Notes';

  @override
  String get instanceDetail => 'Instance details';

  @override
  String get instanceArgs => 'Launch arguments';

  @override
  String get instanceMinMemory => 'Min memory (MB)';

  @override
  String get instanceMaxMemory => 'Max memory (MB)';

  @override
  String get instanceExtraJvm => 'Extra JVM args';

  @override
  String get instanceExtraMc => 'Extra game args';

  @override
  String get instanceWindowWidth => 'Window width';

  @override
  String get instanceWindowHeight => 'Window height';

  @override
  String get instanceLogs => 'Game logs';

  @override
  String get instanceLaunch => 'Launch';

  @override
  String get modsTitle => 'Mods';

  @override
  String get modsEnabled => 'Enabled';

  @override
  String get modsDisabled => 'Disabled';

  @override
  String get modsUpdates => 'Updates available';

  @override
  String get modsCheckUpdates => 'Check updates';

  @override
  String get modsSearch => 'Search Modrinth';

  @override
  String get modsInstallFile => 'Install from file';

  @override
  String get modsConflicts => 'Conflicts';

  @override
  String get modsEmpty => 'No mods installed';

  @override
  String get modsInstall => 'Install';

  @override
  String get modsVersion => 'Version';

  @override
  String get modsDependencies => 'Dependencies';

  @override
  String get modsUpdate => 'Update';

  @override
  String get modsRemove => 'Remove';

  @override
  String get modsSearchPlaceholder => 'Search mods…';

  @override
  String get modsNoResults => 'No results';

  @override
  String get modsLoadingResults => 'Searching…';

  @override
  String modsDownloadCount(Object count) {
    return '$count downloads';
  }

  @override
  String get downloadsTitle => 'Downloads';

  @override
  String get downloadsPause => 'Pause';

  @override
  String get downloadsResume => 'Resume';

  @override
  String get downloadsCancel => 'Cancel';

  @override
  String get downloadsClearFinished => 'Clear finished';

  @override
  String get downloadsEmpty => 'No download tasks';

  @override
  String get downloadsInstallModpack => 'Install modpack';

  @override
  String get downloadsStateQueued => 'Queued';

  @override
  String get downloadsStateRunning => 'Downloading';

  @override
  String get downloadsStatePaused => 'Paused';

  @override
  String get downloadsStateDone => 'Done';

  @override
  String get downloadsStateFailed => 'Failed';

  @override
  String get downloadsStateCanceled => 'Canceled';

  @override
  String downloadsSpeed(Object speed) {
    return '$speed/s';
  }

  @override
  String get settingsTitle => 'Settings';

  @override
  String get settingsAccounts => 'Accounts';

  @override
  String get settingsMirrors => 'Mirrors & sources';

  @override
  String get settingsJava => 'Java';

  @override
  String get settingsGeneral => 'General';

  @override
  String get settingsAbout => 'About';

  @override
  String get settingsLanguage => 'Language';

  @override
  String get settingsThemeMode => 'Theme mode';

  @override
  String get settingsThemeModeSystem => 'System';

  @override
  String get settingsThemeModeLight => 'Light';

  @override
  String get settingsThemeModeDark => 'Dark';

  @override
  String get settingsThemeSeed => 'Theme color';

  @override
  String get settingsAutoUpdate => 'Auto-update launcher';

  @override
  String get settingsDownloadSource => 'Download source';

  @override
  String get settingsSourceOfficial => 'Official';

  @override
  String get settingsSourceBmclapi => 'BMCLAPI';

  @override
  String get settingsSourceCustom => 'Custom';

  @override
  String get settingsCustomHost => 'Custom host';

  @override
  String get settingsLogin => 'Sign in';

  @override
  String get settingsLogout => 'Sign out';

  @override
  String get settingsActive => 'Active';

  @override
  String get settingsMicrosoftLogin => 'Microsoft';

  @override
  String get settingsOfflineLogin => 'Offline';

  @override
  String get settingsYggdrasilLogin => 'Yggdrasil';

  @override
  String get settingsRefreshAccount => 'Refresh';

  @override
  String get settingsScanJava => 'Scan system';

  @override
  String get settingsAddManualJava => 'Add path';

  @override
  String get settingsDownloadJava => 'Download';

  @override
  String get settingsRemoveJava => 'Remove';

  @override
  String get settingsJavaMajor => 'Major';

  @override
  String get settingsJavaPath => 'Path';

  @override
  String get settingsJavaVendor => 'Vendor';

  @override
  String get settingsJavaVersion => 'Version';

  @override
  String settingsAboutText(Object version) {
    return 'Yuhina launcher, version $version';
  }

  @override
  String settingsUpdateAvailable(Object version) {
    return 'Update available: $version';
  }

  @override
  String get settingsUpToDate => 'Up to date';

  @override
  String get logsTitle => 'Game logs';

  @override
  String get logsLevel => 'Level';

  @override
  String get logsLevelInfo => 'Info';

  @override
  String get logsLevelWarn => 'Warning';

  @override
  String get logsLevelError => 'Error';

  @override
  String get logsLevelDebug => 'Debug';

  @override
  String get logsCrashSummary => 'Crash summary';

  @override
  String get logsOpenFile => 'Open log file';

  @override
  String get logsEmpty => 'No log output yet';

  @override
  String get logsState => 'Session state';

  @override
  String get logsStateRunning => 'Running';

  @override
  String get logsStateStopped => 'Stopped';

  @override
  String get logsStateCrashed => 'Crashed';

  @override
  String get authMicrosoftHint =>
      'A browser window will open. Sign in there, then return here.';

  @override
  String get authMicrosoftWaiting => 'Waiting for authorization…';

  @override
  String get authMicrosoftCancel => 'Cancel login';

  @override
  String get authOfflineName => 'Player name';

  @override
  String get authOfflineHint =>
      'Any name works; an offline UUID is generated automatically.';

  @override
  String get authYggdrasilServer => 'Server URL';

  @override
  String get authYggdrasilPreset => 'Presets';

  @override
  String get authYggdrasilLittleSkin => 'LittleSkin';

  @override
  String get authLoginButton => 'Sign in';

  @override
  String authLoginSuccess(Object name) {
    return 'Signed in as $name';
  }

  @override
  String get errorNetwork => 'Network error';

  @override
  String errorHttp(Object status) {
    return 'HTTP error $status';
  }

  @override
  String get errorAuth => 'Authentication failed';

  @override
  String get errorAuthExpired => 'Session expired, please sign in again';

  @override
  String get errorNotLoggedIn => 'Not signed in';

  @override
  String get errorVersionNotFound => 'Version not found';

  @override
  String get errorLoaderNotInstalled => 'Loader installation failed';

  @override
  String get errorJavaNotFound => 'Java not found';

  @override
  String get errorInvalidInstance => 'Invalid instance';

  @override
  String get errorModConflict => 'Mod conflict';

  @override
  String get errorModpackInvalid => 'Invalid modpack';

  @override
  String get errorChecksumMismatch => 'Checksum mismatch';

  @override
  String get errorDownloadFailed => 'Download failed';

  @override
  String get errorCanceled => 'Canceled';

  @override
  String get errorIo => 'File system error';

  @override
  String get errorInternal => 'Internal error';
}
