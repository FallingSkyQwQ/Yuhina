// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Chinese (`zh`).
class AppLocalizationsZh extends AppLocalizations {
  AppLocalizationsZh([String locale = 'zh']) : super(locale);

  @override
  String get appTitle => 'Yuhina 启动器';

  @override
  String get navHome => '首页';

  @override
  String get navInstances => '实例';

  @override
  String get navDownloads => '下载';

  @override
  String get settings => '设置';

  @override
  String get logs => '日志';

  @override
  String get commonCancel => '取消';

  @override
  String get commonConfirm => '确定';

  @override
  String get commonDelete => '删除';

  @override
  String get commonSave => '保存';

  @override
  String get commonClose => '关闭';

  @override
  String get commonSearch => '搜索';

  @override
  String get commonRefresh => '刷新';

  @override
  String get commonLoading => '加载中…';

  @override
  String get commonRetry => '重试';

  @override
  String get commonError => '错误';

  @override
  String get commonEmpty => '暂无内容';

  @override
  String get commonBack => '返回';

  @override
  String get commonOk => '确定';

  @override
  String get commonCopy => '复制';

  @override
  String get commonName => '名称';

  @override
  String get commonOpen => '打开';

  @override
  String get homeQuickLaunch => '快速启动';

  @override
  String get homeNews => '资讯';

  @override
  String get homeNewsUnavailable => '资讯加载失败';

  @override
  String get homeActiveAccount => '当前账号';

  @override
  String get homeNoAccount => '未登录';

  @override
  String get homeNoVersion => '暂无版本列表缓存';

  @override
  String get homeFetchVersions => '获取版本列表';

  @override
  String get instancesTitle => '实例库';

  @override
  String get instancesNew => '新建实例';

  @override
  String get instancesPlay => '启动';

  @override
  String get instancesEdit => '编辑';

  @override
  String get instancesClone => '复制';

  @override
  String get instancesRename => '重命名';

  @override
  String get instancesDelete => '删除';

  @override
  String get instancesOpenDir => '打开目录';

  @override
  String get instancesInstallLoader => '安装加载器';

  @override
  String get instancesMods => 'Mod';

  @override
  String get instancesEmpty => '还没有实例，点击右上角新建一个吧。';

  @override
  String get instancesNotInstalled => '未安装';

  @override
  String get instancesInstalled => '已安装';

  @override
  String instancesLastLaunched(Object time) {
    return '上次启动 $time';
  }

  @override
  String get instancesNeverLaunched => '从未启动';

  @override
  String instancesModCount(Object count) {
    return '$count 个 Mod';
  }

  @override
  String instancesSize(Object size) {
    return '$size';
  }

  @override
  String get instanceNameLabel => '名称';

  @override
  String get instanceIconLabel => '图标';

  @override
  String get instanceMcVersionLabel => 'Minecraft 版本';

  @override
  String get instanceLoaderLabel => '加载器';

  @override
  String get instanceLoaderNone => '原版';

  @override
  String get instanceCreate => '创建';

  @override
  String instanceDeleteConfirm(Object name) {
    return '确定永久删除实例“$name”吗？';
  }

  @override
  String get instanceDeleteFiles => '同时删除游戏文件';

  @override
  String get instanceJavaLabel => 'Java';

  @override
  String instanceJavaAuto(Object major) {
    return '自动（主版本 $major）';
  }

  @override
  String instanceJavaManual(Object path) {
    return '手动（$path）';
  }

  @override
  String get instanceNotes => '备注';

  @override
  String get instanceDetail => '实例详情';

  @override
  String get instanceArgs => '启动参数';

  @override
  String get instanceMinMemory => '最小内存（MB）';

  @override
  String get instanceMaxMemory => '最大内存（MB）';

  @override
  String get instanceExtraJvm => '额外 JVM 参数';

  @override
  String get instanceExtraMc => '额外游戏参数';

  @override
  String get instanceWindowWidth => '窗口宽度';

  @override
  String get instanceWindowHeight => '窗口高度';

  @override
  String get instanceLogs => '游戏日志';

  @override
  String get instanceLaunch => '启动游戏';

  @override
  String get modsTitle => 'Mod 管理';

  @override
  String get modsEnabled => '已启用';

  @override
  String get modsDisabled => '已停用';

  @override
  String get modsUpdates => '有可用更新';

  @override
  String get modsCheckUpdates => '检查更新';

  @override
  String get modsSearch => '搜索 Modrinth';

  @override
  String get modsInstallFile => '从文件安装';

  @override
  String get modsConflicts => '冲突检测';

  @override
  String get modsEmpty => '未安装任何 Mod';

  @override
  String get modsInstall => '安装';

  @override
  String get modsVersion => '版本';

  @override
  String get modsDependencies => '依赖';

  @override
  String get modsUpdate => '更新';

  @override
  String get modsRemove => '移除';

  @override
  String get modsSearchPlaceholder => '搜索 Mod…';

  @override
  String get modsNoResults => '没有结果';

  @override
  String get modsLoadingResults => '搜索中…';

  @override
  String modsDownloadCount(Object count) {
    return '$count 次下载';
  }

  @override
  String get downloadsTitle => '下载中心';

  @override
  String get downloadsPause => '暂停';

  @override
  String get downloadsResume => '恢复';

  @override
  String get downloadsCancel => '取消';

  @override
  String get downloadsClearFinished => '清除已完成';

  @override
  String get downloadsEmpty => '暂无下载任务';

  @override
  String get downloadsInstallModpack => '安装整合包';

  @override
  String get downloadsStateQueued => '排队中';

  @override
  String get downloadsStateRunning => '下载中';

  @override
  String get downloadsStatePaused => '已暂停';

  @override
  String get downloadsStateDone => '完成';

  @override
  String get downloadsStateFailed => '失败';

  @override
  String get downloadsStateCanceled => '已取消';

  @override
  String downloadsSpeed(Object speed) {
    return '$speed/s';
  }

  @override
  String get settingsTitle => '设置';

  @override
  String get settingsAccounts => '账号';

  @override
  String get settingsMirrors => '镜像与源';

  @override
  String get settingsJava => 'Java';

  @override
  String get settingsGeneral => '常规';

  @override
  String get settingsAbout => '关于';

  @override
  String get settingsLanguage => '语言';

  @override
  String get settingsThemeMode => '主题模式';

  @override
  String get settingsThemeModeSystem => '跟随系统';

  @override
  String get settingsThemeModeLight => '浅色';

  @override
  String get settingsThemeModeDark => '深色';

  @override
  String get settingsThemeSeed => '主题色';

  @override
  String get settingsAutoUpdate => '启动器自动更新';

  @override
  String get settingsDownloadSource => '下载源';

  @override
  String get settingsSourceOfficial => '官方源';

  @override
  String get settingsSourceBmclapi => 'BMCLAPI 镜像';

  @override
  String get settingsSourceCustom => '自定义';

  @override
  String get settingsCustomHost => '自定义源地址';

  @override
  String get settingsLogin => '登录';

  @override
  String get settingsLogout => '退出登录';

  @override
  String get settingsActive => '当前使用';

  @override
  String get settingsMicrosoftLogin => '微软';

  @override
  String get settingsOfflineLogin => '离线';

  @override
  String get settingsYggdrasilLogin => 'Yggdrasil';

  @override
  String get settingsRefreshAccount => '刷新';

  @override
  String get settingsScanJava => '扫描系统';

  @override
  String get settingsAddManualJava => '手动添加';

  @override
  String get settingsDownloadJava => '下载';

  @override
  String get settingsRemoveJava => '移除';

  @override
  String get settingsJavaMajor => '主版本';

  @override
  String get settingsJavaPath => '路径';

  @override
  String get settingsJavaVendor => '厂商';

  @override
  String get settingsJavaVersion => '版本';

  @override
  String settingsAboutText(Object version) {
    return 'Yuhina 启动器，版本 $version';
  }

  @override
  String settingsUpdateAvailable(Object version) {
    return '有新版本：$version';
  }

  @override
  String get settingsUpToDate => '已是最新版本';

  @override
  String get logsTitle => '游戏日志';

  @override
  String get logsLevel => '级别';

  @override
  String get logsLevelInfo => '信息';

  @override
  String get logsLevelWarn => '警告';

  @override
  String get logsLevelError => '错误';

  @override
  String get logsLevelDebug => '调试';

  @override
  String get logsCrashSummary => '崩溃摘要';

  @override
  String get logsOpenFile => '打开日志文件';

  @override
  String get logsEmpty => '暂无日志输出';

  @override
  String get logsState => '会话状态';

  @override
  String get logsStateRunning => '运行中';

  @override
  String get logsStateStopped => '已停止';

  @override
  String get logsStateCrashed => '已崩溃';

  @override
  String get authMicrosoftHint => '浏览器窗口即将打开，请在浏览器中登录后返回此界面。';

  @override
  String get authMicrosoftWaiting => '等待授权…';

  @override
  String get authMicrosoftCancel => '取消登录';

  @override
  String get authOfflineName => '玩家名';

  @override
  String get authOfflineHint => '任意名称均可，将自动生成离线 UUID。';

  @override
  String get authYggdrasilServer => '服务器地址';

  @override
  String get authYggdrasilPreset => '预设';

  @override
  String get authYggdrasilLittleSkin => 'LittleSkin';

  @override
  String get authLoginButton => '登录';

  @override
  String authLoginSuccess(Object name) {
    return '已登录 $name';
  }

  @override
  String get errorNetwork => '网络错误';

  @override
  String errorHttp(Object status) {
    return 'HTTP 错误 $status';
  }

  @override
  String get errorAuth => '认证失败';

  @override
  String get errorAuthExpired => '会话已过期，请重新登录';

  @override
  String get errorNotLoggedIn => '尚未登录';

  @override
  String get errorVersionNotFound => '未找到该版本';

  @override
  String get errorLoaderNotInstalled => '加载器安装失败';

  @override
  String get errorJavaNotFound => '未找到 Java';

  @override
  String get errorInvalidInstance => '实例无效';

  @override
  String get errorModConflict => 'Mod 冲突';

  @override
  String get errorModpackInvalid => '整合包无效';

  @override
  String get errorChecksumMismatch => '校验和不匹配';

  @override
  String get errorDownloadFailed => '下载失败';

  @override
  String get errorCanceled => '已取消';

  @override
  String get errorIo => '文件系统错误';

  @override
  String get errorInternal => '内部错误';
}
