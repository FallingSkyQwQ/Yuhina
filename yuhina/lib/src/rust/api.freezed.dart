// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$AppEvent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() configChanged,
    required TResult Function() accountsChanged,
    required TResult Function() instancesChanged,
    required TResult Function(String field0) taskChanged,
    required TResult Function() javaRuntimesChanged,
    required TResult Function() versionListChanged,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? configChanged,
    TResult? Function()? accountsChanged,
    TResult? Function()? instancesChanged,
    TResult? Function(String field0)? taskChanged,
    TResult? Function()? javaRuntimesChanged,
    TResult? Function()? versionListChanged,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? configChanged,
    TResult Function()? accountsChanged,
    TResult Function()? instancesChanged,
    TResult Function(String field0)? taskChanged,
    TResult Function()? javaRuntimesChanged,
    TResult Function()? versionListChanged,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppEvent_ConfigChanged value) configChanged,
    required TResult Function(AppEvent_AccountsChanged value) accountsChanged,
    required TResult Function(AppEvent_InstancesChanged value) instancesChanged,
    required TResult Function(AppEvent_TaskChanged value) taskChanged,
    required TResult Function(AppEvent_JavaRuntimesChanged value)
    javaRuntimesChanged,
    required TResult Function(AppEvent_VersionListChanged value)
    versionListChanged,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppEvent_ConfigChanged value)? configChanged,
    TResult? Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult? Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult? Function(AppEvent_TaskChanged value)? taskChanged,
    TResult? Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult? Function(AppEvent_VersionListChanged value)? versionListChanged,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppEvent_ConfigChanged value)? configChanged,
    TResult Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult Function(AppEvent_TaskChanged value)? taskChanged,
    TResult Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult Function(AppEvent_VersionListChanged value)? versionListChanged,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AppEventCopyWith<$Res> {
  factory $AppEventCopyWith(AppEvent value, $Res Function(AppEvent) then) =
      _$AppEventCopyWithImpl<$Res, AppEvent>;
}

/// @nodoc
class _$AppEventCopyWithImpl<$Res, $Val extends AppEvent>
    implements $AppEventCopyWith<$Res> {
  _$AppEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of AppEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$AppEvent_ConfigChangedImplCopyWith<$Res> {
  factory _$$AppEvent_ConfigChangedImplCopyWith(
    _$AppEvent_ConfigChangedImpl value,
    $Res Function(_$AppEvent_ConfigChangedImpl) then,
  ) = __$$AppEvent_ConfigChangedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppEvent_ConfigChangedImplCopyWithImpl<$Res>
    extends _$AppEventCopyWithImpl<$Res, _$AppEvent_ConfigChangedImpl>
    implements _$$AppEvent_ConfigChangedImplCopyWith<$Res> {
  __$$AppEvent_ConfigChangedImplCopyWithImpl(
    _$AppEvent_ConfigChangedImpl _value,
    $Res Function(_$AppEvent_ConfigChangedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppEvent_ConfigChangedImpl extends AppEvent_ConfigChanged {
  const _$AppEvent_ConfigChangedImpl() : super._();

  @override
  String toString() {
    return 'AppEvent.configChanged()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppEvent_ConfigChangedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() configChanged,
    required TResult Function() accountsChanged,
    required TResult Function() instancesChanged,
    required TResult Function(String field0) taskChanged,
    required TResult Function() javaRuntimesChanged,
    required TResult Function() versionListChanged,
  }) {
    return configChanged();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? configChanged,
    TResult? Function()? accountsChanged,
    TResult? Function()? instancesChanged,
    TResult? Function(String field0)? taskChanged,
    TResult? Function()? javaRuntimesChanged,
    TResult? Function()? versionListChanged,
  }) {
    return configChanged?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? configChanged,
    TResult Function()? accountsChanged,
    TResult Function()? instancesChanged,
    TResult Function(String field0)? taskChanged,
    TResult Function()? javaRuntimesChanged,
    TResult Function()? versionListChanged,
    required TResult orElse(),
  }) {
    if (configChanged != null) {
      return configChanged();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppEvent_ConfigChanged value) configChanged,
    required TResult Function(AppEvent_AccountsChanged value) accountsChanged,
    required TResult Function(AppEvent_InstancesChanged value) instancesChanged,
    required TResult Function(AppEvent_TaskChanged value) taskChanged,
    required TResult Function(AppEvent_JavaRuntimesChanged value)
    javaRuntimesChanged,
    required TResult Function(AppEvent_VersionListChanged value)
    versionListChanged,
  }) {
    return configChanged(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppEvent_ConfigChanged value)? configChanged,
    TResult? Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult? Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult? Function(AppEvent_TaskChanged value)? taskChanged,
    TResult? Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult? Function(AppEvent_VersionListChanged value)? versionListChanged,
  }) {
    return configChanged?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppEvent_ConfigChanged value)? configChanged,
    TResult Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult Function(AppEvent_TaskChanged value)? taskChanged,
    TResult Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult Function(AppEvent_VersionListChanged value)? versionListChanged,
    required TResult orElse(),
  }) {
    if (configChanged != null) {
      return configChanged(this);
    }
    return orElse();
  }
}

abstract class AppEvent_ConfigChanged extends AppEvent {
  const factory AppEvent_ConfigChanged() = _$AppEvent_ConfigChangedImpl;
  const AppEvent_ConfigChanged._() : super._();
}

/// @nodoc
abstract class _$$AppEvent_AccountsChangedImplCopyWith<$Res> {
  factory _$$AppEvent_AccountsChangedImplCopyWith(
    _$AppEvent_AccountsChangedImpl value,
    $Res Function(_$AppEvent_AccountsChangedImpl) then,
  ) = __$$AppEvent_AccountsChangedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppEvent_AccountsChangedImplCopyWithImpl<$Res>
    extends _$AppEventCopyWithImpl<$Res, _$AppEvent_AccountsChangedImpl>
    implements _$$AppEvent_AccountsChangedImplCopyWith<$Res> {
  __$$AppEvent_AccountsChangedImplCopyWithImpl(
    _$AppEvent_AccountsChangedImpl _value,
    $Res Function(_$AppEvent_AccountsChangedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppEvent_AccountsChangedImpl extends AppEvent_AccountsChanged {
  const _$AppEvent_AccountsChangedImpl() : super._();

  @override
  String toString() {
    return 'AppEvent.accountsChanged()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppEvent_AccountsChangedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() configChanged,
    required TResult Function() accountsChanged,
    required TResult Function() instancesChanged,
    required TResult Function(String field0) taskChanged,
    required TResult Function() javaRuntimesChanged,
    required TResult Function() versionListChanged,
  }) {
    return accountsChanged();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? configChanged,
    TResult? Function()? accountsChanged,
    TResult? Function()? instancesChanged,
    TResult? Function(String field0)? taskChanged,
    TResult? Function()? javaRuntimesChanged,
    TResult? Function()? versionListChanged,
  }) {
    return accountsChanged?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? configChanged,
    TResult Function()? accountsChanged,
    TResult Function()? instancesChanged,
    TResult Function(String field0)? taskChanged,
    TResult Function()? javaRuntimesChanged,
    TResult Function()? versionListChanged,
    required TResult orElse(),
  }) {
    if (accountsChanged != null) {
      return accountsChanged();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppEvent_ConfigChanged value) configChanged,
    required TResult Function(AppEvent_AccountsChanged value) accountsChanged,
    required TResult Function(AppEvent_InstancesChanged value) instancesChanged,
    required TResult Function(AppEvent_TaskChanged value) taskChanged,
    required TResult Function(AppEvent_JavaRuntimesChanged value)
    javaRuntimesChanged,
    required TResult Function(AppEvent_VersionListChanged value)
    versionListChanged,
  }) {
    return accountsChanged(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppEvent_ConfigChanged value)? configChanged,
    TResult? Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult? Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult? Function(AppEvent_TaskChanged value)? taskChanged,
    TResult? Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult? Function(AppEvent_VersionListChanged value)? versionListChanged,
  }) {
    return accountsChanged?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppEvent_ConfigChanged value)? configChanged,
    TResult Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult Function(AppEvent_TaskChanged value)? taskChanged,
    TResult Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult Function(AppEvent_VersionListChanged value)? versionListChanged,
    required TResult orElse(),
  }) {
    if (accountsChanged != null) {
      return accountsChanged(this);
    }
    return orElse();
  }
}

abstract class AppEvent_AccountsChanged extends AppEvent {
  const factory AppEvent_AccountsChanged() = _$AppEvent_AccountsChangedImpl;
  const AppEvent_AccountsChanged._() : super._();
}

/// @nodoc
abstract class _$$AppEvent_InstancesChangedImplCopyWith<$Res> {
  factory _$$AppEvent_InstancesChangedImplCopyWith(
    _$AppEvent_InstancesChangedImpl value,
    $Res Function(_$AppEvent_InstancesChangedImpl) then,
  ) = __$$AppEvent_InstancesChangedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppEvent_InstancesChangedImplCopyWithImpl<$Res>
    extends _$AppEventCopyWithImpl<$Res, _$AppEvent_InstancesChangedImpl>
    implements _$$AppEvent_InstancesChangedImplCopyWith<$Res> {
  __$$AppEvent_InstancesChangedImplCopyWithImpl(
    _$AppEvent_InstancesChangedImpl _value,
    $Res Function(_$AppEvent_InstancesChangedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppEvent_InstancesChangedImpl extends AppEvent_InstancesChanged {
  const _$AppEvent_InstancesChangedImpl() : super._();

  @override
  String toString() {
    return 'AppEvent.instancesChanged()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppEvent_InstancesChangedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() configChanged,
    required TResult Function() accountsChanged,
    required TResult Function() instancesChanged,
    required TResult Function(String field0) taskChanged,
    required TResult Function() javaRuntimesChanged,
    required TResult Function() versionListChanged,
  }) {
    return instancesChanged();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? configChanged,
    TResult? Function()? accountsChanged,
    TResult? Function()? instancesChanged,
    TResult? Function(String field0)? taskChanged,
    TResult? Function()? javaRuntimesChanged,
    TResult? Function()? versionListChanged,
  }) {
    return instancesChanged?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? configChanged,
    TResult Function()? accountsChanged,
    TResult Function()? instancesChanged,
    TResult Function(String field0)? taskChanged,
    TResult Function()? javaRuntimesChanged,
    TResult Function()? versionListChanged,
    required TResult orElse(),
  }) {
    if (instancesChanged != null) {
      return instancesChanged();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppEvent_ConfigChanged value) configChanged,
    required TResult Function(AppEvent_AccountsChanged value) accountsChanged,
    required TResult Function(AppEvent_InstancesChanged value) instancesChanged,
    required TResult Function(AppEvent_TaskChanged value) taskChanged,
    required TResult Function(AppEvent_JavaRuntimesChanged value)
    javaRuntimesChanged,
    required TResult Function(AppEvent_VersionListChanged value)
    versionListChanged,
  }) {
    return instancesChanged(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppEvent_ConfigChanged value)? configChanged,
    TResult? Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult? Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult? Function(AppEvent_TaskChanged value)? taskChanged,
    TResult? Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult? Function(AppEvent_VersionListChanged value)? versionListChanged,
  }) {
    return instancesChanged?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppEvent_ConfigChanged value)? configChanged,
    TResult Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult Function(AppEvent_TaskChanged value)? taskChanged,
    TResult Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult Function(AppEvent_VersionListChanged value)? versionListChanged,
    required TResult orElse(),
  }) {
    if (instancesChanged != null) {
      return instancesChanged(this);
    }
    return orElse();
  }
}

abstract class AppEvent_InstancesChanged extends AppEvent {
  const factory AppEvent_InstancesChanged() = _$AppEvent_InstancesChangedImpl;
  const AppEvent_InstancesChanged._() : super._();
}

/// @nodoc
abstract class _$$AppEvent_TaskChangedImplCopyWith<$Res> {
  factory _$$AppEvent_TaskChangedImplCopyWith(
    _$AppEvent_TaskChangedImpl value,
    $Res Function(_$AppEvent_TaskChangedImpl) then,
  ) = __$$AppEvent_TaskChangedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$AppEvent_TaskChangedImplCopyWithImpl<$Res>
    extends _$AppEventCopyWithImpl<$Res, _$AppEvent_TaskChangedImpl>
    implements _$$AppEvent_TaskChangedImplCopyWith<$Res> {
  __$$AppEvent_TaskChangedImplCopyWithImpl(
    _$AppEvent_TaskChangedImpl _value,
    $Res Function(_$AppEvent_TaskChangedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$AppEvent_TaskChangedImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AppEvent_TaskChangedImpl extends AppEvent_TaskChanged {
  const _$AppEvent_TaskChangedImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'AppEvent.taskChanged(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppEvent_TaskChangedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of AppEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AppEvent_TaskChangedImplCopyWith<_$AppEvent_TaskChangedImpl>
  get copyWith =>
      __$$AppEvent_TaskChangedImplCopyWithImpl<_$AppEvent_TaskChangedImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() configChanged,
    required TResult Function() accountsChanged,
    required TResult Function() instancesChanged,
    required TResult Function(String field0) taskChanged,
    required TResult Function() javaRuntimesChanged,
    required TResult Function() versionListChanged,
  }) {
    return taskChanged(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? configChanged,
    TResult? Function()? accountsChanged,
    TResult? Function()? instancesChanged,
    TResult? Function(String field0)? taskChanged,
    TResult? Function()? javaRuntimesChanged,
    TResult? Function()? versionListChanged,
  }) {
    return taskChanged?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? configChanged,
    TResult Function()? accountsChanged,
    TResult Function()? instancesChanged,
    TResult Function(String field0)? taskChanged,
    TResult Function()? javaRuntimesChanged,
    TResult Function()? versionListChanged,
    required TResult orElse(),
  }) {
    if (taskChanged != null) {
      return taskChanged(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppEvent_ConfigChanged value) configChanged,
    required TResult Function(AppEvent_AccountsChanged value) accountsChanged,
    required TResult Function(AppEvent_InstancesChanged value) instancesChanged,
    required TResult Function(AppEvent_TaskChanged value) taskChanged,
    required TResult Function(AppEvent_JavaRuntimesChanged value)
    javaRuntimesChanged,
    required TResult Function(AppEvent_VersionListChanged value)
    versionListChanged,
  }) {
    return taskChanged(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppEvent_ConfigChanged value)? configChanged,
    TResult? Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult? Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult? Function(AppEvent_TaskChanged value)? taskChanged,
    TResult? Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult? Function(AppEvent_VersionListChanged value)? versionListChanged,
  }) {
    return taskChanged?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppEvent_ConfigChanged value)? configChanged,
    TResult Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult Function(AppEvent_TaskChanged value)? taskChanged,
    TResult Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult Function(AppEvent_VersionListChanged value)? versionListChanged,
    required TResult orElse(),
  }) {
    if (taskChanged != null) {
      return taskChanged(this);
    }
    return orElse();
  }
}

abstract class AppEvent_TaskChanged extends AppEvent {
  const factory AppEvent_TaskChanged(String field0) =
      _$AppEvent_TaskChangedImpl;
  const AppEvent_TaskChanged._() : super._();

  String get field0;

  /// Create a copy of AppEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AppEvent_TaskChangedImplCopyWith<_$AppEvent_TaskChangedImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AppEvent_JavaRuntimesChangedImplCopyWith<$Res> {
  factory _$$AppEvent_JavaRuntimesChangedImplCopyWith(
    _$AppEvent_JavaRuntimesChangedImpl value,
    $Res Function(_$AppEvent_JavaRuntimesChangedImpl) then,
  ) = __$$AppEvent_JavaRuntimesChangedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppEvent_JavaRuntimesChangedImplCopyWithImpl<$Res>
    extends _$AppEventCopyWithImpl<$Res, _$AppEvent_JavaRuntimesChangedImpl>
    implements _$$AppEvent_JavaRuntimesChangedImplCopyWith<$Res> {
  __$$AppEvent_JavaRuntimesChangedImplCopyWithImpl(
    _$AppEvent_JavaRuntimesChangedImpl _value,
    $Res Function(_$AppEvent_JavaRuntimesChangedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppEvent_JavaRuntimesChangedImpl extends AppEvent_JavaRuntimesChanged {
  const _$AppEvent_JavaRuntimesChangedImpl() : super._();

  @override
  String toString() {
    return 'AppEvent.javaRuntimesChanged()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppEvent_JavaRuntimesChangedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() configChanged,
    required TResult Function() accountsChanged,
    required TResult Function() instancesChanged,
    required TResult Function(String field0) taskChanged,
    required TResult Function() javaRuntimesChanged,
    required TResult Function() versionListChanged,
  }) {
    return javaRuntimesChanged();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? configChanged,
    TResult? Function()? accountsChanged,
    TResult? Function()? instancesChanged,
    TResult? Function(String field0)? taskChanged,
    TResult? Function()? javaRuntimesChanged,
    TResult? Function()? versionListChanged,
  }) {
    return javaRuntimesChanged?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? configChanged,
    TResult Function()? accountsChanged,
    TResult Function()? instancesChanged,
    TResult Function(String field0)? taskChanged,
    TResult Function()? javaRuntimesChanged,
    TResult Function()? versionListChanged,
    required TResult orElse(),
  }) {
    if (javaRuntimesChanged != null) {
      return javaRuntimesChanged();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppEvent_ConfigChanged value) configChanged,
    required TResult Function(AppEvent_AccountsChanged value) accountsChanged,
    required TResult Function(AppEvent_InstancesChanged value) instancesChanged,
    required TResult Function(AppEvent_TaskChanged value) taskChanged,
    required TResult Function(AppEvent_JavaRuntimesChanged value)
    javaRuntimesChanged,
    required TResult Function(AppEvent_VersionListChanged value)
    versionListChanged,
  }) {
    return javaRuntimesChanged(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppEvent_ConfigChanged value)? configChanged,
    TResult? Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult? Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult? Function(AppEvent_TaskChanged value)? taskChanged,
    TResult? Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult? Function(AppEvent_VersionListChanged value)? versionListChanged,
  }) {
    return javaRuntimesChanged?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppEvent_ConfigChanged value)? configChanged,
    TResult Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult Function(AppEvent_TaskChanged value)? taskChanged,
    TResult Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult Function(AppEvent_VersionListChanged value)? versionListChanged,
    required TResult orElse(),
  }) {
    if (javaRuntimesChanged != null) {
      return javaRuntimesChanged(this);
    }
    return orElse();
  }
}

abstract class AppEvent_JavaRuntimesChanged extends AppEvent {
  const factory AppEvent_JavaRuntimesChanged() =
      _$AppEvent_JavaRuntimesChangedImpl;
  const AppEvent_JavaRuntimesChanged._() : super._();
}

/// @nodoc
abstract class _$$AppEvent_VersionListChangedImplCopyWith<$Res> {
  factory _$$AppEvent_VersionListChangedImplCopyWith(
    _$AppEvent_VersionListChangedImpl value,
    $Res Function(_$AppEvent_VersionListChangedImpl) then,
  ) = __$$AppEvent_VersionListChangedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppEvent_VersionListChangedImplCopyWithImpl<$Res>
    extends _$AppEventCopyWithImpl<$Res, _$AppEvent_VersionListChangedImpl>
    implements _$$AppEvent_VersionListChangedImplCopyWith<$Res> {
  __$$AppEvent_VersionListChangedImplCopyWithImpl(
    _$AppEvent_VersionListChangedImpl _value,
    $Res Function(_$AppEvent_VersionListChangedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppEvent_VersionListChangedImpl extends AppEvent_VersionListChanged {
  const _$AppEvent_VersionListChangedImpl() : super._();

  @override
  String toString() {
    return 'AppEvent.versionListChanged()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppEvent_VersionListChangedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() configChanged,
    required TResult Function() accountsChanged,
    required TResult Function() instancesChanged,
    required TResult Function(String field0) taskChanged,
    required TResult Function() javaRuntimesChanged,
    required TResult Function() versionListChanged,
  }) {
    return versionListChanged();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? configChanged,
    TResult? Function()? accountsChanged,
    TResult? Function()? instancesChanged,
    TResult? Function(String field0)? taskChanged,
    TResult? Function()? javaRuntimesChanged,
    TResult? Function()? versionListChanged,
  }) {
    return versionListChanged?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? configChanged,
    TResult Function()? accountsChanged,
    TResult Function()? instancesChanged,
    TResult Function(String field0)? taskChanged,
    TResult Function()? javaRuntimesChanged,
    TResult Function()? versionListChanged,
    required TResult orElse(),
  }) {
    if (versionListChanged != null) {
      return versionListChanged();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppEvent_ConfigChanged value) configChanged,
    required TResult Function(AppEvent_AccountsChanged value) accountsChanged,
    required TResult Function(AppEvent_InstancesChanged value) instancesChanged,
    required TResult Function(AppEvent_TaskChanged value) taskChanged,
    required TResult Function(AppEvent_JavaRuntimesChanged value)
    javaRuntimesChanged,
    required TResult Function(AppEvent_VersionListChanged value)
    versionListChanged,
  }) {
    return versionListChanged(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppEvent_ConfigChanged value)? configChanged,
    TResult? Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult? Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult? Function(AppEvent_TaskChanged value)? taskChanged,
    TResult? Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult? Function(AppEvent_VersionListChanged value)? versionListChanged,
  }) {
    return versionListChanged?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppEvent_ConfigChanged value)? configChanged,
    TResult Function(AppEvent_AccountsChanged value)? accountsChanged,
    TResult Function(AppEvent_InstancesChanged value)? instancesChanged,
    TResult Function(AppEvent_TaskChanged value)? taskChanged,
    TResult Function(AppEvent_JavaRuntimesChanged value)? javaRuntimesChanged,
    TResult Function(AppEvent_VersionListChanged value)? versionListChanged,
    required TResult orElse(),
  }) {
    if (versionListChanged != null) {
      return versionListChanged(this);
    }
    return orElse();
  }
}

abstract class AppEvent_VersionListChanged extends AppEvent {
  const factory AppEvent_VersionListChanged() =
      _$AppEvent_VersionListChangedImpl;
  const AppEvent_VersionListChanged._() : super._();
}

/// @nodoc
mixin _$GameState {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() starting,
    required TResult Function() running,
    required TResult Function(int field0) stopped,
    required TResult Function(String field0) crashed,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? starting,
    TResult? Function()? running,
    TResult? Function(int field0)? stopped,
    TResult? Function(String field0)? crashed,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? starting,
    TResult Function()? running,
    TResult Function(int field0)? stopped,
    TResult Function(String field0)? crashed,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GameState_Starting value) starting,
    required TResult Function(GameState_Running value) running,
    required TResult Function(GameState_Stopped value) stopped,
    required TResult Function(GameState_Crashed value) crashed,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GameState_Starting value)? starting,
    TResult? Function(GameState_Running value)? running,
    TResult? Function(GameState_Stopped value)? stopped,
    TResult? Function(GameState_Crashed value)? crashed,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GameState_Starting value)? starting,
    TResult Function(GameState_Running value)? running,
    TResult Function(GameState_Stopped value)? stopped,
    TResult Function(GameState_Crashed value)? crashed,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $GameStateCopyWith<$Res> {
  factory $GameStateCopyWith(GameState value, $Res Function(GameState) then) =
      _$GameStateCopyWithImpl<$Res, GameState>;
}

/// @nodoc
class _$GameStateCopyWithImpl<$Res, $Val extends GameState>
    implements $GameStateCopyWith<$Res> {
  _$GameStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of GameState
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$GameState_StartingImplCopyWith<$Res> {
  factory _$$GameState_StartingImplCopyWith(
    _$GameState_StartingImpl value,
    $Res Function(_$GameState_StartingImpl) then,
  ) = __$$GameState_StartingImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$GameState_StartingImplCopyWithImpl<$Res>
    extends _$GameStateCopyWithImpl<$Res, _$GameState_StartingImpl>
    implements _$$GameState_StartingImplCopyWith<$Res> {
  __$$GameState_StartingImplCopyWithImpl(
    _$GameState_StartingImpl _value,
    $Res Function(_$GameState_StartingImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GameState
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$GameState_StartingImpl extends GameState_Starting {
  const _$GameState_StartingImpl() : super._();

  @override
  String toString() {
    return 'GameState.starting()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$GameState_StartingImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() starting,
    required TResult Function() running,
    required TResult Function(int field0) stopped,
    required TResult Function(String field0) crashed,
  }) {
    return starting();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? starting,
    TResult? Function()? running,
    TResult? Function(int field0)? stopped,
    TResult? Function(String field0)? crashed,
  }) {
    return starting?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? starting,
    TResult Function()? running,
    TResult Function(int field0)? stopped,
    TResult Function(String field0)? crashed,
    required TResult orElse(),
  }) {
    if (starting != null) {
      return starting();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GameState_Starting value) starting,
    required TResult Function(GameState_Running value) running,
    required TResult Function(GameState_Stopped value) stopped,
    required TResult Function(GameState_Crashed value) crashed,
  }) {
    return starting(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GameState_Starting value)? starting,
    TResult? Function(GameState_Running value)? running,
    TResult? Function(GameState_Stopped value)? stopped,
    TResult? Function(GameState_Crashed value)? crashed,
  }) {
    return starting?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GameState_Starting value)? starting,
    TResult Function(GameState_Running value)? running,
    TResult Function(GameState_Stopped value)? stopped,
    TResult Function(GameState_Crashed value)? crashed,
    required TResult orElse(),
  }) {
    if (starting != null) {
      return starting(this);
    }
    return orElse();
  }
}

abstract class GameState_Starting extends GameState {
  const factory GameState_Starting() = _$GameState_StartingImpl;
  const GameState_Starting._() : super._();
}

/// @nodoc
abstract class _$$GameState_RunningImplCopyWith<$Res> {
  factory _$$GameState_RunningImplCopyWith(
    _$GameState_RunningImpl value,
    $Res Function(_$GameState_RunningImpl) then,
  ) = __$$GameState_RunningImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$GameState_RunningImplCopyWithImpl<$Res>
    extends _$GameStateCopyWithImpl<$Res, _$GameState_RunningImpl>
    implements _$$GameState_RunningImplCopyWith<$Res> {
  __$$GameState_RunningImplCopyWithImpl(
    _$GameState_RunningImpl _value,
    $Res Function(_$GameState_RunningImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GameState
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$GameState_RunningImpl extends GameState_Running {
  const _$GameState_RunningImpl() : super._();

  @override
  String toString() {
    return 'GameState.running()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$GameState_RunningImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() starting,
    required TResult Function() running,
    required TResult Function(int field0) stopped,
    required TResult Function(String field0) crashed,
  }) {
    return running();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? starting,
    TResult? Function()? running,
    TResult? Function(int field0)? stopped,
    TResult? Function(String field0)? crashed,
  }) {
    return running?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? starting,
    TResult Function()? running,
    TResult Function(int field0)? stopped,
    TResult Function(String field0)? crashed,
    required TResult orElse(),
  }) {
    if (running != null) {
      return running();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GameState_Starting value) starting,
    required TResult Function(GameState_Running value) running,
    required TResult Function(GameState_Stopped value) stopped,
    required TResult Function(GameState_Crashed value) crashed,
  }) {
    return running(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GameState_Starting value)? starting,
    TResult? Function(GameState_Running value)? running,
    TResult? Function(GameState_Stopped value)? stopped,
    TResult? Function(GameState_Crashed value)? crashed,
  }) {
    return running?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GameState_Starting value)? starting,
    TResult Function(GameState_Running value)? running,
    TResult Function(GameState_Stopped value)? stopped,
    TResult Function(GameState_Crashed value)? crashed,
    required TResult orElse(),
  }) {
    if (running != null) {
      return running(this);
    }
    return orElse();
  }
}

abstract class GameState_Running extends GameState {
  const factory GameState_Running() = _$GameState_RunningImpl;
  const GameState_Running._() : super._();
}

/// @nodoc
abstract class _$$GameState_StoppedImplCopyWith<$Res> {
  factory _$$GameState_StoppedImplCopyWith(
    _$GameState_StoppedImpl value,
    $Res Function(_$GameState_StoppedImpl) then,
  ) = __$$GameState_StoppedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$GameState_StoppedImplCopyWithImpl<$Res>
    extends _$GameStateCopyWithImpl<$Res, _$GameState_StoppedImpl>
    implements _$$GameState_StoppedImplCopyWith<$Res> {
  __$$GameState_StoppedImplCopyWithImpl(
    _$GameState_StoppedImpl _value,
    $Res Function(_$GameState_StoppedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GameState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$GameState_StoppedImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$GameState_StoppedImpl extends GameState_Stopped {
  const _$GameState_StoppedImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'GameState.stopped(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GameState_StoppedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of GameState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GameState_StoppedImplCopyWith<_$GameState_StoppedImpl> get copyWith =>
      __$$GameState_StoppedImplCopyWithImpl<_$GameState_StoppedImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() starting,
    required TResult Function() running,
    required TResult Function(int field0) stopped,
    required TResult Function(String field0) crashed,
  }) {
    return stopped(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? starting,
    TResult? Function()? running,
    TResult? Function(int field0)? stopped,
    TResult? Function(String field0)? crashed,
  }) {
    return stopped?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? starting,
    TResult Function()? running,
    TResult Function(int field0)? stopped,
    TResult Function(String field0)? crashed,
    required TResult orElse(),
  }) {
    if (stopped != null) {
      return stopped(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GameState_Starting value) starting,
    required TResult Function(GameState_Running value) running,
    required TResult Function(GameState_Stopped value) stopped,
    required TResult Function(GameState_Crashed value) crashed,
  }) {
    return stopped(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GameState_Starting value)? starting,
    TResult? Function(GameState_Running value)? running,
    TResult? Function(GameState_Stopped value)? stopped,
    TResult? Function(GameState_Crashed value)? crashed,
  }) {
    return stopped?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GameState_Starting value)? starting,
    TResult Function(GameState_Running value)? running,
    TResult Function(GameState_Stopped value)? stopped,
    TResult Function(GameState_Crashed value)? crashed,
    required TResult orElse(),
  }) {
    if (stopped != null) {
      return stopped(this);
    }
    return orElse();
  }
}

abstract class GameState_Stopped extends GameState {
  const factory GameState_Stopped(int field0) = _$GameState_StoppedImpl;
  const GameState_Stopped._() : super._();

  int get field0;

  /// Create a copy of GameState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GameState_StoppedImplCopyWith<_$GameState_StoppedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$GameState_CrashedImplCopyWith<$Res> {
  factory _$$GameState_CrashedImplCopyWith(
    _$GameState_CrashedImpl value,
    $Res Function(_$GameState_CrashedImpl) then,
  ) = __$$GameState_CrashedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$GameState_CrashedImplCopyWithImpl<$Res>
    extends _$GameStateCopyWithImpl<$Res, _$GameState_CrashedImpl>
    implements _$$GameState_CrashedImplCopyWith<$Res> {
  __$$GameState_CrashedImplCopyWithImpl(
    _$GameState_CrashedImpl _value,
    $Res Function(_$GameState_CrashedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of GameState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$GameState_CrashedImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$GameState_CrashedImpl extends GameState_Crashed {
  const _$GameState_CrashedImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'GameState.crashed(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$GameState_CrashedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of GameState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$GameState_CrashedImplCopyWith<_$GameState_CrashedImpl> get copyWith =>
      __$$GameState_CrashedImplCopyWithImpl<_$GameState_CrashedImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() starting,
    required TResult Function() running,
    required TResult Function(int field0) stopped,
    required TResult Function(String field0) crashed,
  }) {
    return crashed(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? starting,
    TResult? Function()? running,
    TResult? Function(int field0)? stopped,
    TResult? Function(String field0)? crashed,
  }) {
    return crashed?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? starting,
    TResult Function()? running,
    TResult Function(int field0)? stopped,
    TResult Function(String field0)? crashed,
    required TResult orElse(),
  }) {
    if (crashed != null) {
      return crashed(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(GameState_Starting value) starting,
    required TResult Function(GameState_Running value) running,
    required TResult Function(GameState_Stopped value) stopped,
    required TResult Function(GameState_Crashed value) crashed,
  }) {
    return crashed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(GameState_Starting value)? starting,
    TResult? Function(GameState_Running value)? running,
    TResult? Function(GameState_Stopped value)? stopped,
    TResult? Function(GameState_Crashed value)? crashed,
  }) {
    return crashed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(GameState_Starting value)? starting,
    TResult Function(GameState_Running value)? running,
    TResult Function(GameState_Stopped value)? stopped,
    TResult Function(GameState_Crashed value)? crashed,
    required TResult orElse(),
  }) {
    if (crashed != null) {
      return crashed(this);
    }
    return orElse();
  }
}

abstract class GameState_Crashed extends GameState {
  const factory GameState_Crashed(String field0) = _$GameState_CrashedImpl;
  const GameState_Crashed._() : super._();

  String get field0;

  /// Create a copy of GameState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$GameState_CrashedImplCopyWith<_$GameState_CrashedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$JavaSelection {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) auto,
    required TResult Function(String field0) manual,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? auto,
    TResult? Function(String field0)? manual,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? auto,
    TResult Function(String field0)? manual,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JavaSelection_Auto value) auto,
    required TResult Function(JavaSelection_Manual value) manual,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JavaSelection_Auto value)? auto,
    TResult? Function(JavaSelection_Manual value)? manual,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JavaSelection_Auto value)? auto,
    TResult Function(JavaSelection_Manual value)? manual,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $JavaSelectionCopyWith<$Res> {
  factory $JavaSelectionCopyWith(
    JavaSelection value,
    $Res Function(JavaSelection) then,
  ) = _$JavaSelectionCopyWithImpl<$Res, JavaSelection>;
}

/// @nodoc
class _$JavaSelectionCopyWithImpl<$Res, $Val extends JavaSelection>
    implements $JavaSelectionCopyWith<$Res> {
  _$JavaSelectionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of JavaSelection
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$JavaSelection_AutoImplCopyWith<$Res> {
  factory _$$JavaSelection_AutoImplCopyWith(
    _$JavaSelection_AutoImpl value,
    $Res Function(_$JavaSelection_AutoImpl) then,
  ) = __$$JavaSelection_AutoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$JavaSelection_AutoImplCopyWithImpl<$Res>
    extends _$JavaSelectionCopyWithImpl<$Res, _$JavaSelection_AutoImpl>
    implements _$$JavaSelection_AutoImplCopyWith<$Res> {
  __$$JavaSelection_AutoImplCopyWithImpl(
    _$JavaSelection_AutoImpl _value,
    $Res Function(_$JavaSelection_AutoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of JavaSelection
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$JavaSelection_AutoImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$JavaSelection_AutoImpl extends JavaSelection_Auto {
  const _$JavaSelection_AutoImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'JavaSelection.auto(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JavaSelection_AutoImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of JavaSelection
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$JavaSelection_AutoImplCopyWith<_$JavaSelection_AutoImpl> get copyWith =>
      __$$JavaSelection_AutoImplCopyWithImpl<_$JavaSelection_AutoImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) auto,
    required TResult Function(String field0) manual,
  }) {
    return auto(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? auto,
    TResult? Function(String field0)? manual,
  }) {
    return auto?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? auto,
    TResult Function(String field0)? manual,
    required TResult orElse(),
  }) {
    if (auto != null) {
      return auto(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JavaSelection_Auto value) auto,
    required TResult Function(JavaSelection_Manual value) manual,
  }) {
    return auto(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JavaSelection_Auto value)? auto,
    TResult? Function(JavaSelection_Manual value)? manual,
  }) {
    return auto?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JavaSelection_Auto value)? auto,
    TResult Function(JavaSelection_Manual value)? manual,
    required TResult orElse(),
  }) {
    if (auto != null) {
      return auto(this);
    }
    return orElse();
  }
}

abstract class JavaSelection_Auto extends JavaSelection {
  const factory JavaSelection_Auto(int field0) = _$JavaSelection_AutoImpl;
  const JavaSelection_Auto._() : super._();

  @override
  int get field0;

  /// Create a copy of JavaSelection
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$JavaSelection_AutoImplCopyWith<_$JavaSelection_AutoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$JavaSelection_ManualImplCopyWith<$Res> {
  factory _$$JavaSelection_ManualImplCopyWith(
    _$JavaSelection_ManualImpl value,
    $Res Function(_$JavaSelection_ManualImpl) then,
  ) = __$$JavaSelection_ManualImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$JavaSelection_ManualImplCopyWithImpl<$Res>
    extends _$JavaSelectionCopyWithImpl<$Res, _$JavaSelection_ManualImpl>
    implements _$$JavaSelection_ManualImplCopyWith<$Res> {
  __$$JavaSelection_ManualImplCopyWithImpl(
    _$JavaSelection_ManualImpl _value,
    $Res Function(_$JavaSelection_ManualImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of JavaSelection
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$JavaSelection_ManualImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$JavaSelection_ManualImpl extends JavaSelection_Manual {
  const _$JavaSelection_ManualImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'JavaSelection.manual(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JavaSelection_ManualImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of JavaSelection
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$JavaSelection_ManualImplCopyWith<_$JavaSelection_ManualImpl>
  get copyWith =>
      __$$JavaSelection_ManualImplCopyWithImpl<_$JavaSelection_ManualImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) auto,
    required TResult Function(String field0) manual,
  }) {
    return manual(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? auto,
    TResult? Function(String field0)? manual,
  }) {
    return manual?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? auto,
    TResult Function(String field0)? manual,
    required TResult orElse(),
  }) {
    if (manual != null) {
      return manual(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JavaSelection_Auto value) auto,
    required TResult Function(JavaSelection_Manual value) manual,
  }) {
    return manual(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JavaSelection_Auto value)? auto,
    TResult? Function(JavaSelection_Manual value)? manual,
  }) {
    return manual?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JavaSelection_Auto value)? auto,
    TResult Function(JavaSelection_Manual value)? manual,
    required TResult orElse(),
  }) {
    if (manual != null) {
      return manual(this);
    }
    return orElse();
  }
}

abstract class JavaSelection_Manual extends JavaSelection {
  const factory JavaSelection_Manual(String field0) =
      _$JavaSelection_ManualImpl;
  const JavaSelection_Manual._() : super._();

  @override
  String get field0;

  /// Create a copy of JavaSelection
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$JavaSelection_ManualImplCopyWith<_$JavaSelection_ManualImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Source {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() official,
    required TResult Function() bmclapi,
    required TResult Function(String field0) custom,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? official,
    TResult? Function()? bmclapi,
    TResult? Function(String field0)? custom,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? official,
    TResult Function()? bmclapi,
    TResult Function(String field0)? custom,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Source_Official value) official,
    required TResult Function(Source_Bmclapi value) bmclapi,
    required TResult Function(Source_Custom value) custom,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Source_Official value)? official,
    TResult? Function(Source_Bmclapi value)? bmclapi,
    TResult? Function(Source_Custom value)? custom,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Source_Official value)? official,
    TResult Function(Source_Bmclapi value)? bmclapi,
    TResult Function(Source_Custom value)? custom,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SourceCopyWith<$Res> {
  factory $SourceCopyWith(Source value, $Res Function(Source) then) =
      _$SourceCopyWithImpl<$Res, Source>;
}

/// @nodoc
class _$SourceCopyWithImpl<$Res, $Val extends Source>
    implements $SourceCopyWith<$Res> {
  _$SourceCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Source
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$Source_OfficialImplCopyWith<$Res> {
  factory _$$Source_OfficialImplCopyWith(
    _$Source_OfficialImpl value,
    $Res Function(_$Source_OfficialImpl) then,
  ) = __$$Source_OfficialImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Source_OfficialImplCopyWithImpl<$Res>
    extends _$SourceCopyWithImpl<$Res, _$Source_OfficialImpl>
    implements _$$Source_OfficialImplCopyWith<$Res> {
  __$$Source_OfficialImplCopyWithImpl(
    _$Source_OfficialImpl _value,
    $Res Function(_$Source_OfficialImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Source
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$Source_OfficialImpl extends Source_Official {
  const _$Source_OfficialImpl() : super._();

  @override
  String toString() {
    return 'Source.official()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Source_OfficialImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() official,
    required TResult Function() bmclapi,
    required TResult Function(String field0) custom,
  }) {
    return official();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? official,
    TResult? Function()? bmclapi,
    TResult? Function(String field0)? custom,
  }) {
    return official?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? official,
    TResult Function()? bmclapi,
    TResult Function(String field0)? custom,
    required TResult orElse(),
  }) {
    if (official != null) {
      return official();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Source_Official value) official,
    required TResult Function(Source_Bmclapi value) bmclapi,
    required TResult Function(Source_Custom value) custom,
  }) {
    return official(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Source_Official value)? official,
    TResult? Function(Source_Bmclapi value)? bmclapi,
    TResult? Function(Source_Custom value)? custom,
  }) {
    return official?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Source_Official value)? official,
    TResult Function(Source_Bmclapi value)? bmclapi,
    TResult Function(Source_Custom value)? custom,
    required TResult orElse(),
  }) {
    if (official != null) {
      return official(this);
    }
    return orElse();
  }
}

abstract class Source_Official extends Source {
  const factory Source_Official() = _$Source_OfficialImpl;
  const Source_Official._() : super._();
}

/// @nodoc
abstract class _$$Source_BmclapiImplCopyWith<$Res> {
  factory _$$Source_BmclapiImplCopyWith(
    _$Source_BmclapiImpl value,
    $Res Function(_$Source_BmclapiImpl) then,
  ) = __$$Source_BmclapiImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Source_BmclapiImplCopyWithImpl<$Res>
    extends _$SourceCopyWithImpl<$Res, _$Source_BmclapiImpl>
    implements _$$Source_BmclapiImplCopyWith<$Res> {
  __$$Source_BmclapiImplCopyWithImpl(
    _$Source_BmclapiImpl _value,
    $Res Function(_$Source_BmclapiImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Source
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$Source_BmclapiImpl extends Source_Bmclapi {
  const _$Source_BmclapiImpl() : super._();

  @override
  String toString() {
    return 'Source.bmclapi()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Source_BmclapiImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() official,
    required TResult Function() bmclapi,
    required TResult Function(String field0) custom,
  }) {
    return bmclapi();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? official,
    TResult? Function()? bmclapi,
    TResult? Function(String field0)? custom,
  }) {
    return bmclapi?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? official,
    TResult Function()? bmclapi,
    TResult Function(String field0)? custom,
    required TResult orElse(),
  }) {
    if (bmclapi != null) {
      return bmclapi();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Source_Official value) official,
    required TResult Function(Source_Bmclapi value) bmclapi,
    required TResult Function(Source_Custom value) custom,
  }) {
    return bmclapi(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Source_Official value)? official,
    TResult? Function(Source_Bmclapi value)? bmclapi,
    TResult? Function(Source_Custom value)? custom,
  }) {
    return bmclapi?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Source_Official value)? official,
    TResult Function(Source_Bmclapi value)? bmclapi,
    TResult Function(Source_Custom value)? custom,
    required TResult orElse(),
  }) {
    if (bmclapi != null) {
      return bmclapi(this);
    }
    return orElse();
  }
}

abstract class Source_Bmclapi extends Source {
  const factory Source_Bmclapi() = _$Source_BmclapiImpl;
  const Source_Bmclapi._() : super._();
}

/// @nodoc
abstract class _$$Source_CustomImplCopyWith<$Res> {
  factory _$$Source_CustomImplCopyWith(
    _$Source_CustomImpl value,
    $Res Function(_$Source_CustomImpl) then,
  ) = __$$Source_CustomImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Source_CustomImplCopyWithImpl<$Res>
    extends _$SourceCopyWithImpl<$Res, _$Source_CustomImpl>
    implements _$$Source_CustomImplCopyWith<$Res> {
  __$$Source_CustomImplCopyWithImpl(
    _$Source_CustomImpl _value,
    $Res Function(_$Source_CustomImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Source
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$Source_CustomImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$Source_CustomImpl extends Source_Custom {
  const _$Source_CustomImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'Source.custom(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Source_CustomImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of Source
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$Source_CustomImplCopyWith<_$Source_CustomImpl> get copyWith =>
      __$$Source_CustomImplCopyWithImpl<_$Source_CustomImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() official,
    required TResult Function() bmclapi,
    required TResult Function(String field0) custom,
  }) {
    return custom(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? official,
    TResult? Function()? bmclapi,
    TResult? Function(String field0)? custom,
  }) {
    return custom?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? official,
    TResult Function()? bmclapi,
    TResult Function(String field0)? custom,
    required TResult orElse(),
  }) {
    if (custom != null) {
      return custom(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Source_Official value) official,
    required TResult Function(Source_Bmclapi value) bmclapi,
    required TResult Function(Source_Custom value) custom,
  }) {
    return custom(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Source_Official value)? official,
    TResult? Function(Source_Bmclapi value)? bmclapi,
    TResult? Function(Source_Custom value)? custom,
  }) {
    return custom?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Source_Official value)? official,
    TResult Function(Source_Bmclapi value)? bmclapi,
    TResult Function(Source_Custom value)? custom,
    required TResult orElse(),
  }) {
    if (custom != null) {
      return custom(this);
    }
    return orElse();
  }
}

abstract class Source_Custom extends Source {
  const factory Source_Custom(String field0) = _$Source_CustomImpl;
  const Source_Custom._() : super._();

  String get field0;

  /// Create a copy of Source
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$Source_CustomImplCopyWith<_$Source_CustomImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$YuhinaErrorKind {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $YuhinaErrorKindCopyWith<$Res> {
  factory $YuhinaErrorKindCopyWith(
    YuhinaErrorKind value,
    $Res Function(YuhinaErrorKind) then,
  ) = _$YuhinaErrorKindCopyWithImpl<$Res, YuhinaErrorKind>;
}

/// @nodoc
class _$YuhinaErrorKindCopyWithImpl<$Res, $Val extends YuhinaErrorKind>
    implements $YuhinaErrorKindCopyWith<$Res> {
  _$YuhinaErrorKindCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$YuhinaErrorKind_NetworkImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_NetworkImplCopyWith(
    _$YuhinaErrorKind_NetworkImpl value,
    $Res Function(_$YuhinaErrorKind_NetworkImpl) then,
  ) = __$$YuhinaErrorKind_NetworkImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_NetworkImplCopyWithImpl<$Res>
    extends _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_NetworkImpl>
    implements _$$YuhinaErrorKind_NetworkImplCopyWith<$Res> {
  __$$YuhinaErrorKind_NetworkImplCopyWithImpl(
    _$YuhinaErrorKind_NetworkImpl _value,
    $Res Function(_$YuhinaErrorKind_NetworkImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_NetworkImpl extends YuhinaErrorKind_Network {
  const _$YuhinaErrorKind_NetworkImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.network()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_NetworkImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return network();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return network?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (network != null) {
      return network();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return network(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return network?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (network != null) {
      return network(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_Network extends YuhinaErrorKind {
  const factory YuhinaErrorKind_Network() = _$YuhinaErrorKind_NetworkImpl;
  const YuhinaErrorKind_Network._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_HttpImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_HttpImplCopyWith(
    _$YuhinaErrorKind_HttpImpl value,
    $Res Function(_$YuhinaErrorKind_HttpImpl) then,
  ) = __$$YuhinaErrorKind_HttpImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0, String field1});
}

/// @nodoc
class __$$YuhinaErrorKind_HttpImplCopyWithImpl<$Res>
    extends _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_HttpImpl>
    implements _$$YuhinaErrorKind_HttpImplCopyWith<$Res> {
  __$$YuhinaErrorKind_HttpImplCopyWithImpl(
    _$YuhinaErrorKind_HttpImpl _value,
    $Res Function(_$YuhinaErrorKind_HttpImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null, Object? field1 = null}) {
    return _then(
      _$YuhinaErrorKind_HttpImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as int,
        null == field1
            ? _value.field1
            : field1 // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$YuhinaErrorKind_HttpImpl extends YuhinaErrorKind_Http {
  const _$YuhinaErrorKind_HttpImpl(this.field0, this.field1) : super._();

  @override
  final int field0;
  @override
  final String field1;

  @override
  String toString() {
    return 'YuhinaErrorKind.http(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_HttpImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$YuhinaErrorKind_HttpImplCopyWith<_$YuhinaErrorKind_HttpImpl>
  get copyWith =>
      __$$YuhinaErrorKind_HttpImplCopyWithImpl<_$YuhinaErrorKind_HttpImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return http(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return http?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (http != null) {
      return http(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return http(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return http?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (http != null) {
      return http(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_Http extends YuhinaErrorKind {
  const factory YuhinaErrorKind_Http(int field0, String field1) =
      _$YuhinaErrorKind_HttpImpl;
  const YuhinaErrorKind_Http._() : super._();

  int get field0;
  String get field1;

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$YuhinaErrorKind_HttpImplCopyWith<_$YuhinaErrorKind_HttpImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$YuhinaErrorKind_AuthImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_AuthImplCopyWith(
    _$YuhinaErrorKind_AuthImpl value,
    $Res Function(_$YuhinaErrorKind_AuthImpl) then,
  ) = __$$YuhinaErrorKind_AuthImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_AuthImplCopyWithImpl<$Res>
    extends _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_AuthImpl>
    implements _$$YuhinaErrorKind_AuthImplCopyWith<$Res> {
  __$$YuhinaErrorKind_AuthImplCopyWithImpl(
    _$YuhinaErrorKind_AuthImpl _value,
    $Res Function(_$YuhinaErrorKind_AuthImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_AuthImpl extends YuhinaErrorKind_Auth {
  const _$YuhinaErrorKind_AuthImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.auth()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_AuthImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return auth();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return auth?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (auth != null) {
      return auth();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return auth(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return auth?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (auth != null) {
      return auth(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_Auth extends YuhinaErrorKind {
  const factory YuhinaErrorKind_Auth() = _$YuhinaErrorKind_AuthImpl;
  const YuhinaErrorKind_Auth._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_AuthExpiredImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_AuthExpiredImplCopyWith(
    _$YuhinaErrorKind_AuthExpiredImpl value,
    $Res Function(_$YuhinaErrorKind_AuthExpiredImpl) then,
  ) = __$$YuhinaErrorKind_AuthExpiredImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_AuthExpiredImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_AuthExpiredImpl>
    implements _$$YuhinaErrorKind_AuthExpiredImplCopyWith<$Res> {
  __$$YuhinaErrorKind_AuthExpiredImplCopyWithImpl(
    _$YuhinaErrorKind_AuthExpiredImpl _value,
    $Res Function(_$YuhinaErrorKind_AuthExpiredImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_AuthExpiredImpl extends YuhinaErrorKind_AuthExpired {
  const _$YuhinaErrorKind_AuthExpiredImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.authExpired()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_AuthExpiredImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return authExpired();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return authExpired?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (authExpired != null) {
      return authExpired();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return authExpired(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return authExpired?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (authExpired != null) {
      return authExpired(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_AuthExpired extends YuhinaErrorKind {
  const factory YuhinaErrorKind_AuthExpired() =
      _$YuhinaErrorKind_AuthExpiredImpl;
  const YuhinaErrorKind_AuthExpired._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_NotLoggedInImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_NotLoggedInImplCopyWith(
    _$YuhinaErrorKind_NotLoggedInImpl value,
    $Res Function(_$YuhinaErrorKind_NotLoggedInImpl) then,
  ) = __$$YuhinaErrorKind_NotLoggedInImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_NotLoggedInImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_NotLoggedInImpl>
    implements _$$YuhinaErrorKind_NotLoggedInImplCopyWith<$Res> {
  __$$YuhinaErrorKind_NotLoggedInImplCopyWithImpl(
    _$YuhinaErrorKind_NotLoggedInImpl _value,
    $Res Function(_$YuhinaErrorKind_NotLoggedInImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_NotLoggedInImpl extends YuhinaErrorKind_NotLoggedIn {
  const _$YuhinaErrorKind_NotLoggedInImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.notLoggedIn()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_NotLoggedInImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return notLoggedIn();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return notLoggedIn?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (notLoggedIn != null) {
      return notLoggedIn();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return notLoggedIn(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return notLoggedIn?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (notLoggedIn != null) {
      return notLoggedIn(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_NotLoggedIn extends YuhinaErrorKind {
  const factory YuhinaErrorKind_NotLoggedIn() =
      _$YuhinaErrorKind_NotLoggedInImpl;
  const YuhinaErrorKind_NotLoggedIn._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_VersionNotFoundImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_VersionNotFoundImplCopyWith(
    _$YuhinaErrorKind_VersionNotFoundImpl value,
    $Res Function(_$YuhinaErrorKind_VersionNotFoundImpl) then,
  ) = __$$YuhinaErrorKind_VersionNotFoundImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_VersionNotFoundImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<
          $Res,
          _$YuhinaErrorKind_VersionNotFoundImpl
        >
    implements _$$YuhinaErrorKind_VersionNotFoundImplCopyWith<$Res> {
  __$$YuhinaErrorKind_VersionNotFoundImplCopyWithImpl(
    _$YuhinaErrorKind_VersionNotFoundImpl _value,
    $Res Function(_$YuhinaErrorKind_VersionNotFoundImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_VersionNotFoundImpl
    extends YuhinaErrorKind_VersionNotFound {
  const _$YuhinaErrorKind_VersionNotFoundImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.versionNotFound()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_VersionNotFoundImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return versionNotFound();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return versionNotFound?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (versionNotFound != null) {
      return versionNotFound();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return versionNotFound(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return versionNotFound?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (versionNotFound != null) {
      return versionNotFound(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_VersionNotFound extends YuhinaErrorKind {
  const factory YuhinaErrorKind_VersionNotFound() =
      _$YuhinaErrorKind_VersionNotFoundImpl;
  const YuhinaErrorKind_VersionNotFound._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_LoaderNotInstalledImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_LoaderNotInstalledImplCopyWith(
    _$YuhinaErrorKind_LoaderNotInstalledImpl value,
    $Res Function(_$YuhinaErrorKind_LoaderNotInstalledImpl) then,
  ) = __$$YuhinaErrorKind_LoaderNotInstalledImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_LoaderNotInstalledImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<
          $Res,
          _$YuhinaErrorKind_LoaderNotInstalledImpl
        >
    implements _$$YuhinaErrorKind_LoaderNotInstalledImplCopyWith<$Res> {
  __$$YuhinaErrorKind_LoaderNotInstalledImplCopyWithImpl(
    _$YuhinaErrorKind_LoaderNotInstalledImpl _value,
    $Res Function(_$YuhinaErrorKind_LoaderNotInstalledImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_LoaderNotInstalledImpl
    extends YuhinaErrorKind_LoaderNotInstalled {
  const _$YuhinaErrorKind_LoaderNotInstalledImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.loaderNotInstalled()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_LoaderNotInstalledImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return loaderNotInstalled();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return loaderNotInstalled?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (loaderNotInstalled != null) {
      return loaderNotInstalled();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return loaderNotInstalled(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return loaderNotInstalled?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (loaderNotInstalled != null) {
      return loaderNotInstalled(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_LoaderNotInstalled extends YuhinaErrorKind {
  const factory YuhinaErrorKind_LoaderNotInstalled() =
      _$YuhinaErrorKind_LoaderNotInstalledImpl;
  const YuhinaErrorKind_LoaderNotInstalled._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_JavaNotFoundImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_JavaNotFoundImplCopyWith(
    _$YuhinaErrorKind_JavaNotFoundImpl value,
    $Res Function(_$YuhinaErrorKind_JavaNotFoundImpl) then,
  ) = __$$YuhinaErrorKind_JavaNotFoundImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_JavaNotFoundImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_JavaNotFoundImpl>
    implements _$$YuhinaErrorKind_JavaNotFoundImplCopyWith<$Res> {
  __$$YuhinaErrorKind_JavaNotFoundImplCopyWithImpl(
    _$YuhinaErrorKind_JavaNotFoundImpl _value,
    $Res Function(_$YuhinaErrorKind_JavaNotFoundImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_JavaNotFoundImpl extends YuhinaErrorKind_JavaNotFound {
  const _$YuhinaErrorKind_JavaNotFoundImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.javaNotFound()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_JavaNotFoundImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return javaNotFound();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return javaNotFound?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (javaNotFound != null) {
      return javaNotFound();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return javaNotFound(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return javaNotFound?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (javaNotFound != null) {
      return javaNotFound(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_JavaNotFound extends YuhinaErrorKind {
  const factory YuhinaErrorKind_JavaNotFound() =
      _$YuhinaErrorKind_JavaNotFoundImpl;
  const YuhinaErrorKind_JavaNotFound._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_InvalidInstanceImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_InvalidInstanceImplCopyWith(
    _$YuhinaErrorKind_InvalidInstanceImpl value,
    $Res Function(_$YuhinaErrorKind_InvalidInstanceImpl) then,
  ) = __$$YuhinaErrorKind_InvalidInstanceImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_InvalidInstanceImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<
          $Res,
          _$YuhinaErrorKind_InvalidInstanceImpl
        >
    implements _$$YuhinaErrorKind_InvalidInstanceImplCopyWith<$Res> {
  __$$YuhinaErrorKind_InvalidInstanceImplCopyWithImpl(
    _$YuhinaErrorKind_InvalidInstanceImpl _value,
    $Res Function(_$YuhinaErrorKind_InvalidInstanceImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_InvalidInstanceImpl
    extends YuhinaErrorKind_InvalidInstance {
  const _$YuhinaErrorKind_InvalidInstanceImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.invalidInstance()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_InvalidInstanceImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return invalidInstance();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return invalidInstance?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (invalidInstance != null) {
      return invalidInstance();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return invalidInstance(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return invalidInstance?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (invalidInstance != null) {
      return invalidInstance(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_InvalidInstance extends YuhinaErrorKind {
  const factory YuhinaErrorKind_InvalidInstance() =
      _$YuhinaErrorKind_InvalidInstanceImpl;
  const YuhinaErrorKind_InvalidInstance._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_ModConflictImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_ModConflictImplCopyWith(
    _$YuhinaErrorKind_ModConflictImpl value,
    $Res Function(_$YuhinaErrorKind_ModConflictImpl) then,
  ) = __$$YuhinaErrorKind_ModConflictImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_ModConflictImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_ModConflictImpl>
    implements _$$YuhinaErrorKind_ModConflictImplCopyWith<$Res> {
  __$$YuhinaErrorKind_ModConflictImplCopyWithImpl(
    _$YuhinaErrorKind_ModConflictImpl _value,
    $Res Function(_$YuhinaErrorKind_ModConflictImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_ModConflictImpl extends YuhinaErrorKind_ModConflict {
  const _$YuhinaErrorKind_ModConflictImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.modConflict()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_ModConflictImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return modConflict();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return modConflict?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (modConflict != null) {
      return modConflict();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return modConflict(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return modConflict?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (modConflict != null) {
      return modConflict(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_ModConflict extends YuhinaErrorKind {
  const factory YuhinaErrorKind_ModConflict() =
      _$YuhinaErrorKind_ModConflictImpl;
  const YuhinaErrorKind_ModConflict._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_ModpackInvalidImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_ModpackInvalidImplCopyWith(
    _$YuhinaErrorKind_ModpackInvalidImpl value,
    $Res Function(_$YuhinaErrorKind_ModpackInvalidImpl) then,
  ) = __$$YuhinaErrorKind_ModpackInvalidImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_ModpackInvalidImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<
          $Res,
          _$YuhinaErrorKind_ModpackInvalidImpl
        >
    implements _$$YuhinaErrorKind_ModpackInvalidImplCopyWith<$Res> {
  __$$YuhinaErrorKind_ModpackInvalidImplCopyWithImpl(
    _$YuhinaErrorKind_ModpackInvalidImpl _value,
    $Res Function(_$YuhinaErrorKind_ModpackInvalidImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_ModpackInvalidImpl
    extends YuhinaErrorKind_ModpackInvalid {
  const _$YuhinaErrorKind_ModpackInvalidImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.modpackInvalid()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_ModpackInvalidImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return modpackInvalid();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return modpackInvalid?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (modpackInvalid != null) {
      return modpackInvalid();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return modpackInvalid(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return modpackInvalid?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (modpackInvalid != null) {
      return modpackInvalid(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_ModpackInvalid extends YuhinaErrorKind {
  const factory YuhinaErrorKind_ModpackInvalid() =
      _$YuhinaErrorKind_ModpackInvalidImpl;
  const YuhinaErrorKind_ModpackInvalid._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_ChecksumMismatchImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_ChecksumMismatchImplCopyWith(
    _$YuhinaErrorKind_ChecksumMismatchImpl value,
    $Res Function(_$YuhinaErrorKind_ChecksumMismatchImpl) then,
  ) = __$$YuhinaErrorKind_ChecksumMismatchImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_ChecksumMismatchImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<
          $Res,
          _$YuhinaErrorKind_ChecksumMismatchImpl
        >
    implements _$$YuhinaErrorKind_ChecksumMismatchImplCopyWith<$Res> {
  __$$YuhinaErrorKind_ChecksumMismatchImplCopyWithImpl(
    _$YuhinaErrorKind_ChecksumMismatchImpl _value,
    $Res Function(_$YuhinaErrorKind_ChecksumMismatchImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_ChecksumMismatchImpl
    extends YuhinaErrorKind_ChecksumMismatch {
  const _$YuhinaErrorKind_ChecksumMismatchImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.checksumMismatch()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_ChecksumMismatchImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return checksumMismatch();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return checksumMismatch?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (checksumMismatch != null) {
      return checksumMismatch();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return checksumMismatch(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return checksumMismatch?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (checksumMismatch != null) {
      return checksumMismatch(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_ChecksumMismatch extends YuhinaErrorKind {
  const factory YuhinaErrorKind_ChecksumMismatch() =
      _$YuhinaErrorKind_ChecksumMismatchImpl;
  const YuhinaErrorKind_ChecksumMismatch._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_DownloadFailedImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_DownloadFailedImplCopyWith(
    _$YuhinaErrorKind_DownloadFailedImpl value,
    $Res Function(_$YuhinaErrorKind_DownloadFailedImpl) then,
  ) = __$$YuhinaErrorKind_DownloadFailedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_DownloadFailedImplCopyWithImpl<$Res>
    extends
        _$YuhinaErrorKindCopyWithImpl<
          $Res,
          _$YuhinaErrorKind_DownloadFailedImpl
        >
    implements _$$YuhinaErrorKind_DownloadFailedImplCopyWith<$Res> {
  __$$YuhinaErrorKind_DownloadFailedImplCopyWithImpl(
    _$YuhinaErrorKind_DownloadFailedImpl _value,
    $Res Function(_$YuhinaErrorKind_DownloadFailedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_DownloadFailedImpl
    extends YuhinaErrorKind_DownloadFailed {
  const _$YuhinaErrorKind_DownloadFailedImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.downloadFailed()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_DownloadFailedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return downloadFailed();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return downloadFailed?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (downloadFailed != null) {
      return downloadFailed();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return downloadFailed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return downloadFailed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (downloadFailed != null) {
      return downloadFailed(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_DownloadFailed extends YuhinaErrorKind {
  const factory YuhinaErrorKind_DownloadFailed() =
      _$YuhinaErrorKind_DownloadFailedImpl;
  const YuhinaErrorKind_DownloadFailed._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_CanceledImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_CanceledImplCopyWith(
    _$YuhinaErrorKind_CanceledImpl value,
    $Res Function(_$YuhinaErrorKind_CanceledImpl) then,
  ) = __$$YuhinaErrorKind_CanceledImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_CanceledImplCopyWithImpl<$Res>
    extends _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_CanceledImpl>
    implements _$$YuhinaErrorKind_CanceledImplCopyWith<$Res> {
  __$$YuhinaErrorKind_CanceledImplCopyWithImpl(
    _$YuhinaErrorKind_CanceledImpl _value,
    $Res Function(_$YuhinaErrorKind_CanceledImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_CanceledImpl extends YuhinaErrorKind_Canceled {
  const _$YuhinaErrorKind_CanceledImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.canceled()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_CanceledImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return canceled();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return canceled?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (canceled != null) {
      return canceled();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return canceled(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return canceled?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (canceled != null) {
      return canceled(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_Canceled extends YuhinaErrorKind {
  const factory YuhinaErrorKind_Canceled() = _$YuhinaErrorKind_CanceledImpl;
  const YuhinaErrorKind_Canceled._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_IoImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_IoImplCopyWith(
    _$YuhinaErrorKind_IoImpl value,
    $Res Function(_$YuhinaErrorKind_IoImpl) then,
  ) = __$$YuhinaErrorKind_IoImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_IoImplCopyWithImpl<$Res>
    extends _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_IoImpl>
    implements _$$YuhinaErrorKind_IoImplCopyWith<$Res> {
  __$$YuhinaErrorKind_IoImplCopyWithImpl(
    _$YuhinaErrorKind_IoImpl _value,
    $Res Function(_$YuhinaErrorKind_IoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_IoImpl extends YuhinaErrorKind_Io {
  const _$YuhinaErrorKind_IoImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.io()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$YuhinaErrorKind_IoImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return io();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return io?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (io != null) {
      return io();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return io(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return io?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (io != null) {
      return io(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_Io extends YuhinaErrorKind {
  const factory YuhinaErrorKind_Io() = _$YuhinaErrorKind_IoImpl;
  const YuhinaErrorKind_Io._() : super._();
}

/// @nodoc
abstract class _$$YuhinaErrorKind_InternalImplCopyWith<$Res> {
  factory _$$YuhinaErrorKind_InternalImplCopyWith(
    _$YuhinaErrorKind_InternalImpl value,
    $Res Function(_$YuhinaErrorKind_InternalImpl) then,
  ) = __$$YuhinaErrorKind_InternalImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$YuhinaErrorKind_InternalImplCopyWithImpl<$Res>
    extends _$YuhinaErrorKindCopyWithImpl<$Res, _$YuhinaErrorKind_InternalImpl>
    implements _$$YuhinaErrorKind_InternalImplCopyWith<$Res> {
  __$$YuhinaErrorKind_InternalImplCopyWithImpl(
    _$YuhinaErrorKind_InternalImpl _value,
    $Res Function(_$YuhinaErrorKind_InternalImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of YuhinaErrorKind
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$YuhinaErrorKind_InternalImpl extends YuhinaErrorKind_Internal {
  const _$YuhinaErrorKind_InternalImpl() : super._();

  @override
  String toString() {
    return 'YuhinaErrorKind.internal()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$YuhinaErrorKind_InternalImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() network,
    required TResult Function(int field0, String field1) http,
    required TResult Function() auth,
    required TResult Function() authExpired,
    required TResult Function() notLoggedIn,
    required TResult Function() versionNotFound,
    required TResult Function() loaderNotInstalled,
    required TResult Function() javaNotFound,
    required TResult Function() invalidInstance,
    required TResult Function() modConflict,
    required TResult Function() modpackInvalid,
    required TResult Function() checksumMismatch,
    required TResult Function() downloadFailed,
    required TResult Function() canceled,
    required TResult Function() io,
    required TResult Function() internal,
  }) {
    return internal();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? network,
    TResult? Function(int field0, String field1)? http,
    TResult? Function()? auth,
    TResult? Function()? authExpired,
    TResult? Function()? notLoggedIn,
    TResult? Function()? versionNotFound,
    TResult? Function()? loaderNotInstalled,
    TResult? Function()? javaNotFound,
    TResult? Function()? invalidInstance,
    TResult? Function()? modConflict,
    TResult? Function()? modpackInvalid,
    TResult? Function()? checksumMismatch,
    TResult? Function()? downloadFailed,
    TResult? Function()? canceled,
    TResult? Function()? io,
    TResult? Function()? internal,
  }) {
    return internal?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? network,
    TResult Function(int field0, String field1)? http,
    TResult Function()? auth,
    TResult Function()? authExpired,
    TResult Function()? notLoggedIn,
    TResult Function()? versionNotFound,
    TResult Function()? loaderNotInstalled,
    TResult Function()? javaNotFound,
    TResult Function()? invalidInstance,
    TResult Function()? modConflict,
    TResult Function()? modpackInvalid,
    TResult Function()? checksumMismatch,
    TResult Function()? downloadFailed,
    TResult Function()? canceled,
    TResult Function()? io,
    TResult Function()? internal,
    required TResult orElse(),
  }) {
    if (internal != null) {
      return internal();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(YuhinaErrorKind_Network value) network,
    required TResult Function(YuhinaErrorKind_Http value) http,
    required TResult Function(YuhinaErrorKind_Auth value) auth,
    required TResult Function(YuhinaErrorKind_AuthExpired value) authExpired,
    required TResult Function(YuhinaErrorKind_NotLoggedIn value) notLoggedIn,
    required TResult Function(YuhinaErrorKind_VersionNotFound value)
    versionNotFound,
    required TResult Function(YuhinaErrorKind_LoaderNotInstalled value)
    loaderNotInstalled,
    required TResult Function(YuhinaErrorKind_JavaNotFound value) javaNotFound,
    required TResult Function(YuhinaErrorKind_InvalidInstance value)
    invalidInstance,
    required TResult Function(YuhinaErrorKind_ModConflict value) modConflict,
    required TResult Function(YuhinaErrorKind_ModpackInvalid value)
    modpackInvalid,
    required TResult Function(YuhinaErrorKind_ChecksumMismatch value)
    checksumMismatch,
    required TResult Function(YuhinaErrorKind_DownloadFailed value)
    downloadFailed,
    required TResult Function(YuhinaErrorKind_Canceled value) canceled,
    required TResult Function(YuhinaErrorKind_Io value) io,
    required TResult Function(YuhinaErrorKind_Internal value) internal,
  }) {
    return internal(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(YuhinaErrorKind_Network value)? network,
    TResult? Function(YuhinaErrorKind_Http value)? http,
    TResult? Function(YuhinaErrorKind_Auth value)? auth,
    TResult? Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult? Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult? Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult? Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult? Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult? Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult? Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult? Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult? Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult? Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult? Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult? Function(YuhinaErrorKind_Io value)? io,
    TResult? Function(YuhinaErrorKind_Internal value)? internal,
  }) {
    return internal?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(YuhinaErrorKind_Network value)? network,
    TResult Function(YuhinaErrorKind_Http value)? http,
    TResult Function(YuhinaErrorKind_Auth value)? auth,
    TResult Function(YuhinaErrorKind_AuthExpired value)? authExpired,
    TResult Function(YuhinaErrorKind_NotLoggedIn value)? notLoggedIn,
    TResult Function(YuhinaErrorKind_VersionNotFound value)? versionNotFound,
    TResult Function(YuhinaErrorKind_LoaderNotInstalled value)?
    loaderNotInstalled,
    TResult Function(YuhinaErrorKind_JavaNotFound value)? javaNotFound,
    TResult Function(YuhinaErrorKind_InvalidInstance value)? invalidInstance,
    TResult Function(YuhinaErrorKind_ModConflict value)? modConflict,
    TResult Function(YuhinaErrorKind_ModpackInvalid value)? modpackInvalid,
    TResult Function(YuhinaErrorKind_ChecksumMismatch value)? checksumMismatch,
    TResult Function(YuhinaErrorKind_DownloadFailed value)? downloadFailed,
    TResult Function(YuhinaErrorKind_Canceled value)? canceled,
    TResult Function(YuhinaErrorKind_Io value)? io,
    TResult Function(YuhinaErrorKind_Internal value)? internal,
    required TResult orElse(),
  }) {
    if (internal != null) {
      return internal(this);
    }
    return orElse();
  }
}

abstract class YuhinaErrorKind_Internal extends YuhinaErrorKind {
  const factory YuhinaErrorKind_Internal() = _$YuhinaErrorKind_InternalImpl;
  const YuhinaErrorKind_Internal._() : super._();
}
