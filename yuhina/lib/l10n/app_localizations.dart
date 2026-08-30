import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_en.dart';
import 'app_localizations_zh.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale)
    : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations)!;
  }

  static const LocalizationsDelegate<AppLocalizations> delegate =
      _AppLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates =
      <LocalizationsDelegate<dynamic>>[
        delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
      ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('en'),
    Locale('zh'),
  ];

  /// No description provided for @appTitle.
  ///
  /// In en, this message translates to:
  /// **'Yuhina Launcher'**
  String get appTitle;

  /// No description provided for @navHome.
  ///
  /// In en, this message translates to:
  /// **'Home'**
  String get navHome;

  /// No description provided for @navInstances.
  ///
  /// In en, this message translates to:
  /// **'Instances'**
  String get navInstances;

  /// No description provided for @navDownloads.
  ///
  /// In en, this message translates to:
  /// **'Downloads'**
  String get navDownloads;

  /// No description provided for @settings.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get settings;

  /// No description provided for @logs.
  ///
  /// In en, this message translates to:
  /// **'Logs'**
  String get logs;

  /// No description provided for @commonCancel.
  ///
  /// In en, this message translates to:
  /// **'Cancel'**
  String get commonCancel;

  /// No description provided for @commonConfirm.
  ///
  /// In en, this message translates to:
  /// **'Confirm'**
  String get commonConfirm;

  /// No description provided for @commonDelete.
  ///
  /// In en, this message translates to:
  /// **'Delete'**
  String get commonDelete;

  /// No description provided for @commonSave.
  ///
  /// In en, this message translates to:
  /// **'Save'**
  String get commonSave;

  /// No description provided for @commonClose.
  ///
  /// In en, this message translates to:
  /// **'Close'**
  String get commonClose;

  /// No description provided for @commonSearch.
  ///
  /// In en, this message translates to:
  /// **'Search'**
  String get commonSearch;

  /// No description provided for @commonRefresh.
  ///
  /// In en, this message translates to:
  /// **'Refresh'**
  String get commonRefresh;

  /// No description provided for @commonLoading.
  ///
  /// In en, this message translates to:
  /// **'Loading…'**
  String get commonLoading;

  /// No description provided for @commonRetry.
  ///
  /// In en, this message translates to:
  /// **'Retry'**
  String get commonRetry;

  /// No description provided for @commonError.
  ///
  /// In en, this message translates to:
  /// **'Error'**
  String get commonError;

  /// No description provided for @commonEmpty.
  ///
  /// In en, this message translates to:
  /// **'Nothing here yet'**
  String get commonEmpty;

  /// No description provided for @commonBack.
  ///
  /// In en, this message translates to:
  /// **'Back'**
  String get commonBack;

  /// No description provided for @commonOk.
  ///
  /// In en, this message translates to:
  /// **'OK'**
  String get commonOk;

  /// No description provided for @commonCopy.
  ///
  /// In en, this message translates to:
  /// **'Copy'**
  String get commonCopy;

  /// No description provided for @commonName.
  ///
  /// In en, this message translates to:
  /// **'Name'**
  String get commonName;

  /// No description provided for @commonOpen.
  ///
  /// In en, this message translates to:
  /// **'Open'**
  String get commonOpen;

  /// No description provided for @homeQuickLaunch.
  ///
  /// In en, this message translates to:
  /// **'Quick Launch'**
  String get homeQuickLaunch;

  /// No description provided for @homeNews.
  ///
  /// In en, this message translates to:
  /// **'News'**
  String get homeNews;

  /// No description provided for @homeNewsUnavailable.
  ///
  /// In en, this message translates to:
  /// **'News unavailable'**
  String get homeNewsUnavailable;

  /// No description provided for @homeActiveAccount.
  ///
  /// In en, this message translates to:
  /// **'Active account'**
  String get homeActiveAccount;

  /// No description provided for @homeNoAccount.
  ///
  /// In en, this message translates to:
  /// **'Not signed in'**
  String get homeNoAccount;

  /// No description provided for @homeNoVersion.
  ///
  /// In en, this message translates to:
  /// **'No cached version list'**
  String get homeNoVersion;

  /// No description provided for @homeFetchVersions.
  ///
  /// In en, this message translates to:
  /// **'Fetch versions'**
  String get homeFetchVersions;

  /// No description provided for @instancesTitle.
  ///
  /// In en, this message translates to:
  /// **'Instances'**
  String get instancesTitle;

  /// No description provided for @instancesNew.
  ///
  /// In en, this message translates to:
  /// **'New instance'**
  String get instancesNew;

  /// No description provided for @instancesPlay.
  ///
  /// In en, this message translates to:
  /// **'Play'**
  String get instancesPlay;

  /// No description provided for @instancesEdit.
  ///
  /// In en, this message translates to:
  /// **'Edit'**
  String get instancesEdit;

  /// No description provided for @instancesClone.
  ///
  /// In en, this message translates to:
  /// **'Clone'**
  String get instancesClone;

  /// No description provided for @instancesRename.
  ///
  /// In en, this message translates to:
  /// **'Rename'**
  String get instancesRename;

  /// No description provided for @instancesDelete.
  ///
  /// In en, this message translates to:
  /// **'Delete'**
  String get instancesDelete;

  /// No description provided for @instancesOpenDir.
  ///
  /// In en, this message translates to:
  /// **'Open folder'**
  String get instancesOpenDir;

  /// No description provided for @instancesInstallLoader.
  ///
  /// In en, this message translates to:
  /// **'Install loader'**
  String get instancesInstallLoader;

  /// No description provided for @instancesMods.
  ///
  /// In en, this message translates to:
  /// **'Mods'**
  String get instancesMods;

  /// No description provided for @instancesEmpty.
  ///
  /// In en, this message translates to:
  /// **'No instances yet. Create one to get started.'**
  String get instancesEmpty;

  /// No description provided for @instancesNotInstalled.
  ///
  /// In en, this message translates to:
  /// **'Not installed'**
  String get instancesNotInstalled;

  /// No description provided for @instancesInstalled.
  ///
  /// In en, this message translates to:
  /// **'Installed'**
  String get instancesInstalled;

  /// No description provided for @instancesLastLaunched.
  ///
  /// In en, this message translates to:
  /// **'Last launched {time}'**
  String instancesLastLaunched(Object time);

  /// No description provided for @instancesNeverLaunched.
  ///
  /// In en, this message translates to:
  /// **'Never launched'**
  String get instancesNeverLaunched;

  /// No description provided for @instancesModCount.
  ///
  /// In en, this message translates to:
  /// **'{count} mods'**
  String instancesModCount(Object count);

  /// No description provided for @instancesSize.
  ///
  /// In en, this message translates to:
  /// **'{size}'**
  String instancesSize(Object size);

  /// No description provided for @instanceNameLabel.
  ///
  /// In en, this message translates to:
  /// **'Name'**
  String get instanceNameLabel;

  /// No description provided for @instanceIconLabel.
  ///
  /// In en, this message translates to:
  /// **'Icon'**
  String get instanceIconLabel;

  /// No description provided for @instanceMcVersionLabel.
  ///
  /// In en, this message translates to:
  /// **'Minecraft version'**
  String get instanceMcVersionLabel;

  /// No description provided for @instanceLoaderLabel.
  ///
  /// In en, this message translates to:
  /// **'Loader'**
  String get instanceLoaderLabel;

  /// No description provided for @instanceLoaderNone.
  ///
  /// In en, this message translates to:
  /// **'Vanilla'**
  String get instanceLoaderNone;

  /// No description provided for @instanceCreate.
  ///
  /// In en, this message translates to:
  /// **'Create'**
  String get instanceCreate;

  /// No description provided for @instanceDeleteConfirm.
  ///
  /// In en, this message translates to:
  /// **'Delete instance “{name}” permanently?'**
  String instanceDeleteConfirm(Object name);

  /// No description provided for @instanceDeleteFiles.
  ///
  /// In en, this message translates to:
  /// **'Also delete game files'**
  String get instanceDeleteFiles;

  /// No description provided for @instanceJavaLabel.
  ///
  /// In en, this message translates to:
  /// **'Java'**
  String get instanceJavaLabel;

  /// No description provided for @instanceJavaAuto.
  ///
  /// In en, this message translates to:
  /// **'Auto (major {major})'**
  String instanceJavaAuto(Object major);

  /// No description provided for @instanceJavaManual.
  ///
  /// In en, this message translates to:
  /// **'Manual ({path})'**
  String instanceJavaManual(Object path);

  /// No description provided for @instanceNotes.
  ///
  /// In en, this message translates to:
  /// **'Notes'**
  String get instanceNotes;

  /// No description provided for @instanceDetail.
  ///
  /// In en, this message translates to:
  /// **'Instance details'**
  String get instanceDetail;

  /// No description provided for @instanceArgs.
  ///
  /// In en, this message translates to:
  /// **'Launch arguments'**
  String get instanceArgs;

  /// No description provided for @instanceMinMemory.
  ///
  /// In en, this message translates to:
  /// **'Min memory (MB)'**
  String get instanceMinMemory;

  /// No description provided for @instanceMaxMemory.
  ///
  /// In en, this message translates to:
  /// **'Max memory (MB)'**
  String get instanceMaxMemory;

  /// No description provided for @instanceExtraJvm.
  ///
  /// In en, this message translates to:
  /// **'Extra JVM args'**
  String get instanceExtraJvm;

  /// No description provided for @instanceExtraMc.
  ///
  /// In en, this message translates to:
  /// **'Extra game args'**
  String get instanceExtraMc;

  /// No description provided for @instanceWindowWidth.
  ///
  /// In en, this message translates to:
  /// **'Window width'**
  String get instanceWindowWidth;

  /// No description provided for @instanceWindowHeight.
  ///
  /// In en, this message translates to:
  /// **'Window height'**
  String get instanceWindowHeight;

  /// No description provided for @instanceLogs.
  ///
  /// In en, this message translates to:
  /// **'Game logs'**
  String get instanceLogs;

  /// No description provided for @instanceLaunch.
  ///
  /// In en, this message translates to:
  /// **'Launch'**
  String get instanceLaunch;

  /// No description provided for @modsTitle.
  ///
  /// In en, this message translates to:
  /// **'Mods'**
  String get modsTitle;

  /// No description provided for @modsEnabled.
  ///
  /// In en, this message translates to:
  /// **'Enabled'**
  String get modsEnabled;

  /// No description provided for @modsDisabled.
  ///
  /// In en, this message translates to:
  /// **'Disabled'**
  String get modsDisabled;

  /// No description provided for @modsUpdates.
  ///
  /// In en, this message translates to:
  /// **'Updates available'**
  String get modsUpdates;

  /// No description provided for @modsCheckUpdates.
  ///
  /// In en, this message translates to:
  /// **'Check updates'**
  String get modsCheckUpdates;

  /// No description provided for @modsSearch.
  ///
  /// In en, this message translates to:
  /// **'Search Modrinth'**
  String get modsSearch;

  /// No description provided for @modsInstallFile.
  ///
  /// In en, this message translates to:
  /// **'Install from file'**
  String get modsInstallFile;

  /// No description provided for @modsConflicts.
  ///
  /// In en, this message translates to:
  /// **'Conflicts'**
  String get modsConflicts;

  /// No description provided for @modsEmpty.
  ///
  /// In en, this message translates to:
  /// **'No mods installed'**
  String get modsEmpty;

  /// No description provided for @modsInstall.
  ///
  /// In en, this message translates to:
  /// **'Install'**
  String get modsInstall;

  /// No description provided for @modsVersion.
  ///
  /// In en, this message translates to:
  /// **'Version'**
  String get modsVersion;

  /// No description provided for @modsDependencies.
  ///
  /// In en, this message translates to:
  /// **'Dependencies'**
  String get modsDependencies;

  /// No description provided for @modsUpdate.
  ///
  /// In en, this message translates to:
  /// **'Update'**
  String get modsUpdate;

  /// No description provided for @modsRemove.
  ///
  /// In en, this message translates to:
  /// **'Remove'**
  String get modsRemove;

  /// No description provided for @modsSearchPlaceholder.
  ///
  /// In en, this message translates to:
  /// **'Search mods…'**
  String get modsSearchPlaceholder;

  /// No description provided for @modsNoResults.
  ///
  /// In en, this message translates to:
  /// **'No results'**
  String get modsNoResults;

  /// No description provided for @modsLoadingResults.
  ///
  /// In en, this message translates to:
  /// **'Searching…'**
  String get modsLoadingResults;

  /// No description provided for @modsDownloadCount.
  ///
  /// In en, this message translates to:
  /// **'{count} downloads'**
  String modsDownloadCount(Object count);

  /// No description provided for @downloadsTitle.
  ///
  /// In en, this message translates to:
  /// **'Downloads'**
  String get downloadsTitle;

  /// No description provided for @downloadsPause.
  ///
  /// In en, this message translates to:
  /// **'Pause'**
  String get downloadsPause;

  /// No description provided for @downloadsResume.
  ///
  /// In en, this message translates to:
  /// **'Resume'**
  String get downloadsResume;

  /// No description provided for @downloadsCancel.
  ///
  /// In en, this message translates to:
  /// **'Cancel'**
  String get downloadsCancel;

  /// No description provided for @downloadsClearFinished.
  ///
  /// In en, this message translates to:
  /// **'Clear finished'**
  String get downloadsClearFinished;

  /// No description provided for @downloadsEmpty.
  ///
  /// In en, this message translates to:
  /// **'No download tasks'**
  String get downloadsEmpty;

  /// No description provided for @downloadsInstallModpack.
  ///
  /// In en, this message translates to:
  /// **'Install modpack'**
  String get downloadsInstallModpack;

  /// No description provided for @downloadsStateQueued.
  ///
  /// In en, this message translates to:
  /// **'Queued'**
  String get downloadsStateQueued;

  /// No description provided for @downloadsStateRunning.
  ///
  /// In en, this message translates to:
  /// **'Downloading'**
  String get downloadsStateRunning;

  /// No description provided for @downloadsStatePaused.
  ///
  /// In en, this message translates to:
  /// **'Paused'**
  String get downloadsStatePaused;

  /// No description provided for @downloadsStateDone.
  ///
  /// In en, this message translates to:
  /// **'Done'**
  String get downloadsStateDone;

  /// No description provided for @downloadsStateFailed.
  ///
  /// In en, this message translates to:
  /// **'Failed'**
  String get downloadsStateFailed;

  /// No description provided for @downloadsStateCanceled.
  ///
  /// In en, this message translates to:
  /// **'Canceled'**
  String get downloadsStateCanceled;

  /// No description provided for @downloadsSpeed.
  ///
  /// In en, this message translates to:
  /// **'{speed}/s'**
  String downloadsSpeed(Object speed);

  /// No description provided for @settingsTitle.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get settingsTitle;

  /// No description provided for @settingsAccounts.
  ///
  /// In en, this message translates to:
  /// **'Accounts'**
  String get settingsAccounts;

  /// No description provided for @settingsMirrors.
  ///
  /// In en, this message translates to:
  /// **'Mirrors & sources'**
  String get settingsMirrors;

  /// No description provided for @settingsJava.
  ///
  /// In en, this message translates to:
  /// **'Java'**
  String get settingsJava;

  /// No description provided for @settingsGeneral.
  ///
  /// In en, this message translates to:
  /// **'General'**
  String get settingsGeneral;

  /// No description provided for @settingsAbout.
  ///
  /// In en, this message translates to:
  /// **'About'**
  String get settingsAbout;

  /// No description provided for @settingsLanguage.
  ///
  /// In en, this message translates to:
  /// **'Language'**
  String get settingsLanguage;

  /// No description provided for @settingsThemeMode.
  ///
  /// In en, this message translates to:
  /// **'Theme mode'**
  String get settingsThemeMode;

  /// No description provided for @settingsThemeModeSystem.
  ///
  /// In en, this message translates to:
  /// **'System'**
  String get settingsThemeModeSystem;

  /// No description provided for @settingsThemeModeLight.
  ///
  /// In en, this message translates to:
  /// **'Light'**
  String get settingsThemeModeLight;

  /// No description provided for @settingsThemeModeDark.
  ///
  /// In en, this message translates to:
  /// **'Dark'**
  String get settingsThemeModeDark;

  /// No description provided for @settingsThemeSeed.
  ///
  /// In en, this message translates to:
  /// **'Theme color'**
  String get settingsThemeSeed;

  /// No description provided for @settingsAutoUpdate.
  ///
  /// In en, this message translates to:
  /// **'Auto-update launcher'**
  String get settingsAutoUpdate;

  /// No description provided for @settingsDownloadSource.
  ///
  /// In en, this message translates to:
  /// **'Download source'**
  String get settingsDownloadSource;

  /// No description provided for @settingsSourceOfficial.
  ///
  /// In en, this message translates to:
  /// **'Official'**
  String get settingsSourceOfficial;

  /// No description provided for @settingsSourceBmclapi.
  ///
  /// In en, this message translates to:
  /// **'BMCLAPI'**
  String get settingsSourceBmclapi;

  /// No description provided for @settingsSourceCustom.
  ///
  /// In en, this message translates to:
  /// **'Custom'**
  String get settingsSourceCustom;

  /// No description provided for @settingsCustomHost.
  ///
  /// In en, this message translates to:
  /// **'Custom host'**
  String get settingsCustomHost;

  /// No description provided for @settingsLogin.
  ///
  /// In en, this message translates to:
  /// **'Sign in'**
  String get settingsLogin;

  /// No description provided for @settingsLogout.
  ///
  /// In en, this message translates to:
  /// **'Sign out'**
  String get settingsLogout;

  /// No description provided for @settingsActive.
  ///
  /// In en, this message translates to:
  /// **'Active'**
  String get settingsActive;

  /// No description provided for @settingsMicrosoftLogin.
  ///
  /// In en, this message translates to:
  /// **'Microsoft'**
  String get settingsMicrosoftLogin;

  /// No description provided for @settingsOfflineLogin.
  ///
  /// In en, this message translates to:
  /// **'Offline'**
  String get settingsOfflineLogin;

  /// No description provided for @settingsYggdrasilLogin.
  ///
  /// In en, this message translates to:
  /// **'Yggdrasil'**
  String get settingsYggdrasilLogin;

  /// No description provided for @settingsRefreshAccount.
  ///
  /// In en, this message translates to:
  /// **'Refresh'**
  String get settingsRefreshAccount;

  /// No description provided for @settingsScanJava.
  ///
  /// In en, this message translates to:
  /// **'Scan system'**
  String get settingsScanJava;

  /// No description provided for @settingsAddManualJava.
  ///
  /// In en, this message translates to:
  /// **'Add path'**
  String get settingsAddManualJava;

  /// No description provided for @settingsDownloadJava.
  ///
  /// In en, this message translates to:
  /// **'Download'**
  String get settingsDownloadJava;

  /// No description provided for @settingsRemoveJava.
  ///
  /// In en, this message translates to:
  /// **'Remove'**
  String get settingsRemoveJava;

  /// No description provided for @settingsJavaMajor.
  ///
  /// In en, this message translates to:
  /// **'Major'**
  String get settingsJavaMajor;

  /// No description provided for @settingsJavaPath.
  ///
  /// In en, this message translates to:
  /// **'Path'**
  String get settingsJavaPath;

  /// No description provided for @settingsJavaVendor.
  ///
  /// In en, this message translates to:
  /// **'Vendor'**
  String get settingsJavaVendor;

  /// No description provided for @settingsJavaVersion.
  ///
  /// In en, this message translates to:
  /// **'Version'**
  String get settingsJavaVersion;

  /// No description provided for @settingsAboutText.
  ///
  /// In en, this message translates to:
  /// **'Yuhina launcher, version {version}'**
  String settingsAboutText(Object version);

  /// No description provided for @settingsUpdateAvailable.
  ///
  /// In en, this message translates to:
  /// **'Update available: {version}'**
  String settingsUpdateAvailable(Object version);

  /// No description provided for @settingsUpToDate.
  ///
  /// In en, this message translates to:
  /// **'Up to date'**
  String get settingsUpToDate;

  /// No description provided for @logsTitle.
  ///
  /// In en, this message translates to:
  /// **'Game logs'**
  String get logsTitle;

  /// No description provided for @logsLevel.
  ///
  /// In en, this message translates to:
  /// **'Level'**
  String get logsLevel;

  /// No description provided for @logsLevelInfo.
  ///
  /// In en, this message translates to:
  /// **'Info'**
  String get logsLevelInfo;

  /// No description provided for @logsLevelWarn.
  ///
  /// In en, this message translates to:
  /// **'Warning'**
  String get logsLevelWarn;

  /// No description provided for @logsLevelError.
  ///
  /// In en, this message translates to:
  /// **'Error'**
  String get logsLevelError;

  /// No description provided for @logsLevelDebug.
  ///
  /// In en, this message translates to:
  /// **'Debug'**
  String get logsLevelDebug;

  /// No description provided for @logsCrashSummary.
  ///
  /// In en, this message translates to:
  /// **'Crash summary'**
  String get logsCrashSummary;

  /// No description provided for @logsOpenFile.
  ///
  /// In en, this message translates to:
  /// **'Open log file'**
  String get logsOpenFile;

  /// No description provided for @logsEmpty.
  ///
  /// In en, this message translates to:
  /// **'No log output yet'**
  String get logsEmpty;

  /// No description provided for @logsState.
  ///
  /// In en, this message translates to:
  /// **'Session state'**
  String get logsState;

  /// No description provided for @logsStateRunning.
  ///
  /// In en, this message translates to:
  /// **'Running'**
  String get logsStateRunning;

  /// No description provided for @logsStateStopped.
  ///
  /// In en, this message translates to:
  /// **'Stopped'**
  String get logsStateStopped;

  /// No description provided for @logsStateCrashed.
  ///
  /// In en, this message translates to:
  /// **'Crashed'**
  String get logsStateCrashed;

  /// No description provided for @authMicrosoftHint.
  ///
  /// In en, this message translates to:
  /// **'A browser window will open. Sign in there, then return here.'**
  String get authMicrosoftHint;

  /// No description provided for @authMicrosoftWaiting.
  ///
  /// In en, this message translates to:
  /// **'Waiting for authorization…'**
  String get authMicrosoftWaiting;

  /// No description provided for @authMicrosoftCancel.
  ///
  /// In en, this message translates to:
  /// **'Cancel login'**
  String get authMicrosoftCancel;

  /// No description provided for @authOfflineName.
  ///
  /// In en, this message translates to:
  /// **'Player name'**
  String get authOfflineName;

  /// No description provided for @authOfflineHint.
  ///
  /// In en, this message translates to:
  /// **'Any name works; an offline UUID is generated automatically.'**
  String get authOfflineHint;

  /// No description provided for @authYggdrasilServer.
  ///
  /// In en, this message translates to:
  /// **'Server URL'**
  String get authYggdrasilServer;

  /// No description provided for @authYggdrasilPreset.
  ///
  /// In en, this message translates to:
  /// **'Presets'**
  String get authYggdrasilPreset;

  /// No description provided for @authYggdrasilLittleSkin.
  ///
  /// In en, this message translates to:
  /// **'LittleSkin'**
  String get authYggdrasilLittleSkin;

  /// No description provided for @authLoginButton.
  ///
  /// In en, this message translates to:
  /// **'Sign in'**
  String get authLoginButton;

  /// No description provided for @authLoginSuccess.
  ///
  /// In en, this message translates to:
  /// **'Signed in as {name}'**
  String authLoginSuccess(Object name);

  /// No description provided for @errorNetwork.
  ///
  /// In en, this message translates to:
  /// **'Network error'**
  String get errorNetwork;

  /// No description provided for @errorHttp.
  ///
  /// In en, this message translates to:
  /// **'HTTP error {status}'**
  String errorHttp(Object status);

  /// No description provided for @errorAuth.
  ///
  /// In en, this message translates to:
  /// **'Authentication failed'**
  String get errorAuth;

  /// No description provided for @errorAuthExpired.
  ///
  /// In en, this message translates to:
  /// **'Session expired, please sign in again'**
  String get errorAuthExpired;

  /// No description provided for @errorNotLoggedIn.
  ///
  /// In en, this message translates to:
  /// **'Not signed in'**
  String get errorNotLoggedIn;

  /// No description provided for @errorVersionNotFound.
  ///
  /// In en, this message translates to:
  /// **'Version not found'**
  String get errorVersionNotFound;

  /// No description provided for @errorLoaderNotInstalled.
  ///
  /// In en, this message translates to:
  /// **'Loader installation failed'**
  String get errorLoaderNotInstalled;

  /// No description provided for @errorJavaNotFound.
  ///
  /// In en, this message translates to:
  /// **'Java not found'**
  String get errorJavaNotFound;

  /// No description provided for @errorInvalidInstance.
  ///
  /// In en, this message translates to:
  /// **'Invalid instance'**
  String get errorInvalidInstance;

  /// No description provided for @errorModConflict.
  ///
  /// In en, this message translates to:
  /// **'Mod conflict'**
  String get errorModConflict;

  /// No description provided for @errorModpackInvalid.
  ///
  /// In en, this message translates to:
  /// **'Invalid modpack'**
  String get errorModpackInvalid;

  /// No description provided for @errorChecksumMismatch.
  ///
  /// In en, this message translates to:
  /// **'Checksum mismatch'**
  String get errorChecksumMismatch;

  /// No description provided for @errorDownloadFailed.
  ///
  /// In en, this message translates to:
  /// **'Download failed'**
  String get errorDownloadFailed;

  /// No description provided for @errorCanceled.
  ///
  /// In en, this message translates to:
  /// **'Canceled'**
  String get errorCanceled;

  /// No description provided for @errorIo.
  ///
  /// In en, this message translates to:
  /// **'File system error'**
  String get errorIo;

  /// No description provided for @errorInternal.
  ///
  /// In en, this message translates to:
  /// **'Internal error'**
  String get errorInternal;
}

class _AppLocalizationsDelegate
    extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) =>
      <String>['en', 'zh'].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {
  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en':
      return AppLocalizationsEn();
    case 'zh':
      return AppLocalizationsZh();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.',
  );
}
