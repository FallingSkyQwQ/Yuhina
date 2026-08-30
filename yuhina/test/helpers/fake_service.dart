// A hand-rolled fake `YuhinaService` for widget/golden tests (no FFI, no Rust
// library load). Unimplemented methods fall back to `noSuchMethod`.

import 'package:yuhina/src/rust/api.dart';
import 'package:yuhina/src/rust/service.dart';
import 'package:yuhina/src/rust/third_party/yuhina_api/error.dart';
import 'package:yuhina/src/rust/third_party/yuhina_api/types.dart';

class FakeYuhinaService implements YuhinaService {
  FakeYuhinaService({
    this.config,
    List<InstanceSummary>? instances,
    List<Account>? accounts,
    List<NewsItem>? news,
    List<VersionMeta>? versions,
  })  : instances = instances ?? [],
        accounts = accounts ?? [],
        news = news ?? [],
        versions = versions ?? [];

  LauncherConfig? config;
  final List<InstanceSummary> instances;
  final List<Account> accounts;
  final List<NewsItem> news;
  final List<VersionMeta> versions;

  @override
  Future<LauncherConfig> getConfig() async =>
      config ??
      LauncherConfig(
        dataDir: '~/.yuhina',
        gameRoot: '~/.yuhina/games',
        downloadSource: const Source.official(),
        customSourceHost: null,
        launchArgs: const LaunchArgs(
          minMemoryMb: 2048,
          maxMemoryMb: 4096,
          extraJvmArgs: [],
          extraMcArgs: [],
          windowWidth: null,
          windowHeight: null,
        ),
        locale: 'zh-CN',
        themeSeed: 0,
        autoUpdate: true,
      );

  @override
  Future<List<InstanceSummary>> listInstances() async => instances;

  @override
  Future<List<Account>> listAccounts() async => accounts;

  @override
  Future<Account> getActiveAccount() async {
    final a = accounts.where((a) => a.isActive).firstOrNull;
    if (a == null) {
      throw YuhinaError(kind: const YuhinaErrorKind_NotLoggedIn(), message: 'not logged in');
    }
    return a;
  }

  @override
  Future<List<NewsItem>> getNews() async => news;

  @override
  Future<List<VersionMeta>> getVersionList() async => versions;

  @override
  Future<List<VersionMeta>> fetchVersionList() async => versions;

  @override
  Stream<AppEvent> watchEvents() => const Stream.empty();

  @override
  Stream<DownloadProgressEvent> watchProgress() => const Stream.empty();

  @override
  Stream<GameOutput> watchGameOutput({required String sessionId}) =>
      const Stream.empty();

  @override
  dynamic noSuchMethod(Invocation invocation) => throw UnimplementedError(
        'FakeYuhinaService: ${invocation.memberName}',
      );
}

/// Convenient sample instance summary.
InstanceSummary sampleInstance({
  String id = 'i1',
  String name = 'Test Instance',
  String icon = '🎮',
  String mcVersion = '1.20.4',
  Loader? loader,
  bool isInstalled = true,
  int? lastLaunchedAt,
}) =>
    InstanceSummary(
      id: id,
      name: name,
      icon: icon,
      mcVersion: mcVersion,
      loader: loader,
      isInstalled: isInstalled,
      lastLaunchedAt: lastLaunchedAt == null ? null : BigInt.from(lastLaunchedAt),
      modCount: 3,
      totalSizeBytes: BigInt.from(1024 * 1024 * 250),
      createdAt: BigInt.zero,
      updatedAt: BigInt.zero,
    );

Account sampleAccount({String name = 'Steve', bool active = true}) => Account(
      id: 'a1',
      kind: AccountKind.offline,
      username: name,
      uuid: '00000000-0000-0000-0000-000000000001',
      yggdrasilServer: null,
      skinUrl: null,
      isActive: active,
      expiresAt: null,
    );