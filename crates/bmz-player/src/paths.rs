use std::env;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};

pub const RESOURCE_PATH_PREFIX: &str = "resource:";
pub const DATA_PATH_PREFIX: &str = "data:";

/// ライブラリへ保存するファイルシステムパスを正準化する。
///
/// Windows の `canonicalize()` が付ける extended-length path prefix は
/// 通常のパス操作や設定表示には不要なため取り除く。UNC パスは
/// `\\?\UNC\server\share` から `//server/share` へ戻す。
pub(crate) fn normalize_library_path(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    let Some(without_prefix) = normalized.strip_prefix("//?/") else {
        return normalized;
    };

    if without_prefix.get(..4).is_some_and(|prefix| prefix.eq_ignore_ascii_case("UNC/")) {
        format!("//{}", &without_prefix[4..])
    } else {
        without_prefix.to_string()
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
const APP_DIR_NAME: &str = "BMZ Player";
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
const UNIX_APP_DIR_NAME: &str = "bmz-player";

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub resource_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub config_toml: PathBuf,
    pub library_db: PathBuf,
    pub profiles_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ProfilePaths {
    pub root_dir: PathBuf,
    pub profile_toml: PathBuf,
    pub collection_db: PathBuf,
    pub score_db: PathBuf,
    pub network_db: PathBuf,
    pub replay_dir: PathBuf,
}

pub fn resolve_app_paths() -> Result<AppPaths> {
    let current_dir = env::current_dir().context("failed to resolve current directory")?;
    let exe_path = env::current_exe().ok();
    let exe_dir = exe_path.as_ref().and_then(|path| path.parent()).map(Path::to_path_buf);
    let development_data_dir = development_workspace_data_dir(exe_path.as_deref());

    let resource_dir = env_path("BMZ_RESOURCE_DIR").unwrap_or_else(|| {
        default_resource_dir(
            &current_dir,
            exe_path.as_deref(),
            exe_dir.as_deref(),
            development_data_dir.as_deref(),
        )
    });
    let ResolvedDataDir { path: data_dir, keep_auxiliary_dirs_with_data } =
        match env_path("BMZ_DATA_DIR") {
            Some(path) => ResolvedDataDir { path, keep_auxiliary_dirs_with_data: true },
            None => {
                default_data_dir(&current_dir, exe_dir.as_deref(), development_data_dir.as_deref())
            }
        };
    let cache_dir = env_path("BMZ_CACHE_DIR")
        .unwrap_or_else(|| default_cache_dir(&data_dir, keep_auxiliary_dirs_with_data));
    let logs_dir = env_path("BMZ_LOGS_DIR")
        .unwrap_or_else(|| default_logs_dir(&data_dir, keep_auxiliary_dirs_with_data));

    Ok(AppPaths::from_dirs(resource_dir, data_dir, cache_dir, logs_dir))
}

pub fn resolve_profile_paths(app: &AppPaths, profile_id: &str) -> Result<ProfilePaths> {
    validate_profile_id(profile_id)?;
    let root_dir = app.profiles_dir.join(profile_id);
    Ok(ProfilePaths {
        profile_toml: root_dir.join("profile.toml"),
        collection_db: root_dir.join("collection.db"),
        score_db: root_dir.join("score.db"),
        network_db: root_dir.join("network.db"),
        replay_dir: root_dir.join("replay"),
        root_dir,
    })
}

pub fn validate_profile_id(profile_id: &str) -> Result<()> {
    if profile_id.is_empty() {
        bail!("profile id must not be empty");
    }

    if profile_id.len() > 64 {
        bail!("profile id must be 64 bytes or less");
    }

    if !profile_id.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    {
        bail!("profile id may only contain ASCII letters, digits, '_' and '-'");
    }

    Ok(())
}

impl AppPaths {
    pub fn from_dirs(
        resource_dir: PathBuf,
        data_dir: PathBuf,
        cache_dir: PathBuf,
        logs_dir: PathBuf,
    ) -> Self {
        Self {
            config_toml: data_dir.join("config.toml"),
            library_db: data_dir.join("library.db"),
            profiles_dir: data_dir.join("profiles"),
            resource_dir,
            data_dir,
            cache_dir,
            logs_dir,
        }
    }

    pub fn ensure_dirs(&self) -> Result<()> {
        self.ensure_required_dirs()
    }

    /// アプリ動作に必須のディレクトリだけを作成する。
    ///
    /// logs は永続loggerのbest-effort初期化で別途作成する。ログ保存先が
    /// read-onlyでも、設定・DB・cacheが利用可能ならアプリ本体は起動できる。
    pub fn ensure_required_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(self.data_dir.join("skins"))?;
        std::fs::create_dir_all(&self.profiles_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        Ok(())
    }

    pub fn default_skin_root(&self) -> PathBuf {
        self.resource_dir.join("skins/default")
    }

    /// Explicit package roots available to beatoraja-compatible skin loaders.
    /// User skins take precedence over bundled packages when both roots exist.
    pub fn skin_library_roots(&self) -> Vec<PathBuf> {
        let data = self.data_dir.join("skins");
        let resource = self.resource_dir.join("skins");
        if same_path(&data, &resource) { vec![data] } else { vec![data, resource] }
    }

    /// OS フォントがなくても UI / スキン文字列を描画できるように同梱する Noto CJK の位置。
    pub fn bundled_noto_cjk_font_root(&self) -> PathBuf {
        self.resource_dir.join("fonts/noto-cjk")
    }

    pub fn hides_bundled_skin_label(&self) -> bool {
        same_path(&self.resource_dir.join("skins"), &self.data_dir.join("skins"))
            || sibling_named_dirs(&self.resource_dir, "resources", &self.data_dir, "data")
    }

    pub fn resolve_path_ref(&self, path_ref: &str) -> Result<PathBuf> {
        let trimmed = path_ref.trim();
        if let Some(relative) = trimmed.strip_prefix(RESOURCE_PATH_PREFIX) {
            return join_checked(&self.resource_dir, relative);
        }
        if let Some(relative) = trimmed.strip_prefix(DATA_PATH_PREFIX) {
            return join_checked(&self.data_dir, relative);
        }

        let path = Path::new(trimmed);
        if path.is_absolute() {
            return Ok(path.to_path_buf());
        }
        if let Some(relative) = strip_first_component(path, "data") {
            if let Some(skin_relative) = strip_first_component(&relative, "skins") {
                let data_candidate = self.data_dir.join("skins").join(&skin_relative);
                if data_candidate.exists() {
                    return Ok(data_candidate);
                }
                let resource_candidate = self.resource_dir.join("skins").join(&skin_relative);
                if resource_candidate.exists() {
                    return Ok(resource_candidate);
                }
                return Ok(data_candidate);
            }
            return Ok(self.data_dir.join(relative));
        }
        Ok(path.to_path_buf())
    }

    pub fn resolve_optional_path_ref(&self, path_ref: &str) -> Result<Option<PathBuf>> {
        if path_ref.trim().is_empty() {
            return Ok(None);
        }
        self.resolve_path_ref(path_ref).map(Some)
    }

    pub fn resource_path_ref(&self, path: &Path) -> Option<String> {
        path.strip_prefix(&self.resource_dir)
            .ok()
            .map(|relative| format!("{RESOURCE_PATH_PREFIX}{}", path_to_slash(relative)))
    }

    pub fn data_path_ref(&self, path: &Path) -> Option<String> {
        path.strip_prefix(&self.data_dir)
            .ok()
            .map(|relative| format!("{DATA_PATH_PREFIX}{}", path_to_slash(relative)))
    }
}

impl ProfilePaths {
    pub fn ensure_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(&self.root_dir)?;
        std::fs::create_dir_all(&self.replay_dir)?;
        Ok(())
    }
}

fn env_path(name: &str) -> Option<PathBuf> {
    env::var_os(name).filter(|value| !value.is_empty()).map(PathBuf::from)
}

#[derive(Debug)]
struct ResolvedDataDir {
    path: PathBuf,
    keep_auxiliary_dirs_with_data: bool,
}

fn default_resource_dir(
    current_dir: &Path,
    exe_path: Option<&Path>,
    exe_dir: Option<&Path>,
    development_data_dir: Option<&Path>,
) -> PathBuf {
    if let Some(resources) = macos_app_resource_dir(exe_path).filter(|path| path.exists()) {
        return resources;
    }
    if let Some(resources) = exe_dir.map(|dir| dir.join("resources")).filter(|path| path.exists()) {
        return resources;
    }

    if let Some(data_dir) = development_data_dir {
        return data_dir.to_path_buf();
    }

    let current_data = current_dir.join("data");
    if current_data.exists() {
        return current_data;
    }

    macos_app_resource_dir(exe_path)
        .or_else(|| exe_dir.map(|dir| dir.join("resources")))
        .unwrap_or(current_data)
}

fn default_data_dir(
    current_dir: &Path,
    exe_dir: Option<&Path>,
    development_data_dir: Option<&Path>,
) -> ResolvedDataDir {
    if let Some(data_dir) = exe_dir.map(|dir| dir.join("data")).filter(|path| path.exists()) {
        return ResolvedDataDir { path: data_dir, keep_auxiliary_dirs_with_data: true };
    }

    if let Some(data_dir) = development_data_dir {
        return ResolvedDataDir {
            path: data_dir.to_path_buf(),
            keep_auxiliary_dirs_with_data: true,
        };
    }

    let current_data = current_dir.join("data");
    if current_data.exists() {
        return ResolvedDataDir { path: current_data, keep_auxiliary_dirs_with_data: true };
    }

    match platform_data_dir() {
        Some(path) => ResolvedDataDir { path, keep_auxiliary_dirs_with_data: false },
        None => ResolvedDataDir { path: current_data, keep_auxiliary_dirs_with_data: true },
    }
}

fn default_cache_dir(data_dir: &Path, keep_auxiliary_dirs_with_data: bool) -> PathBuf {
    if keep_auxiliary_dirs_with_data {
        return data_dir.join("cache");
    }
    platform_cache_dir().unwrap_or_else(|| data_dir.join("cache"))
}

fn default_logs_dir(data_dir: &Path, keep_auxiliary_dirs_with_data: bool) -> PathBuf {
    if keep_auxiliary_dirs_with_data {
        return data_dir.join("logs");
    }
    platform_logs_dir().unwrap_or_else(|| data_dir.join("logs"))
}

/// Cargo がこのワークスペースへ直接出力した開発用実行ファイルだけに、
/// ソースツリー内の data を結び付ける。
///
/// ビルド元の絶対パスだけを信用すると、別の場所へコピーした実行ファイルまで
/// 元のソースツリーを読み書きしてしまうため、実行ファイル自身が同じ
/// `target/debug`、`target/release`、またはその `deps` にある同crateのテスト成果物で
/// あることも正規化後のパスで確認する。
fn development_workspace_data_dir(exe_path: Option<&Path>) -> Option<PathBuf> {
    development_workspace_data_dir_from(
        exe_path?,
        Path::new(env!("CARGO_MANIFEST_DIR")),
        env!("CARGO_PKG_NAME"),
    )
}

fn development_workspace_data_dir_from(
    exe_path: &Path,
    manifest_dir: &Path,
    package_name: &str,
) -> Option<PathBuf> {
    let workspace_dir = manifest_dir.parent()?.parent()?;
    let canonical_workspace_dir = workspace_dir.canonicalize().ok()?;
    let exe_path = exe_path.canonicalize().ok()?;
    let exe_stem = exe_path.file_stem()?.to_str()?;
    let exe_dir = exe_path.parent()?;
    let (profile_dir, is_test_artifact) = if exe_dir.file_name()?.to_str()? == "deps" {
        (exe_dir.parent()?, true)
    } else {
        (exe_dir, false)
    };
    let profile_name = profile_dir.file_name()?.to_str()?;
    if profile_name != "debug" && profile_name != "release" {
        return None;
    }

    if is_test_artifact {
        let crate_stem = package_name.replace('-', "_");
        if exe_stem != crate_stem && !exe_stem.starts_with(&format!("{crate_stem}-")) {
            return None;
        }
    } else if exe_stem != package_name {
        return None;
    }

    let expected_profile_dir =
        canonical_workspace_dir.join("target").join(profile_name).canonicalize().ok()?;
    if profile_dir != expected_profile_dir {
        return None;
    }

    let workspace_manifest = workspace_dir.join("Cargo.toml");
    let package_manifest = manifest_dir.join("Cargo.toml");
    let data_dir = workspace_dir.join("data");
    if !workspace_manifest.is_file() || !package_manifest.is_file() || !data_dir.is_dir() {
        return None;
    }

    Some(data_dir)
}

fn same_path(a: &Path, b: &Path) -> bool {
    if a == b {
        return true;
    }
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn sibling_named_dirs(a: &Path, a_name: &str, b: &Path, b_name: &str) -> bool {
    path_file_name_eq(a, a_name)
        && path_file_name_eq(b, b_name)
        && a.parent()
            .zip(b.parent())
            .is_some_and(|(a_parent, b_parent)| same_path(a_parent, b_parent))
}

fn path_file_name_eq(path: &Path, expected: &str) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case(expected))
}

fn macos_app_resource_dir(exe_path: Option<&Path>) -> Option<PathBuf> {
    let exe_path = exe_path?;
    let macos_dir = exe_path.parent()?;
    if macos_dir.file_name()? != "MacOS" {
        return None;
    }
    let contents_dir = macos_dir.parent()?;
    if contents_dir.file_name()? != "Contents" {
        return None;
    }
    Some(contents_dir.join("Resources"))
}

#[cfg(target_os = "windows")]
fn platform_data_dir() -> Option<PathBuf> {
    env::var_os("APPDATA").map(|base| PathBuf::from(base).join(APP_DIR_NAME))
}

#[cfg(target_os = "macos")]
fn platform_data_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join("Library/Application Support").join(APP_DIR_NAME))
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn platform_data_dir() -> Option<PathBuf> {
    env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .map(|base| base.join(UNIX_APP_DIR_NAME))
}

#[cfg(target_os = "windows")]
fn platform_cache_dir() -> Option<PathBuf> {
    env::var_os("LOCALAPPDATA").map(|base| PathBuf::from(base).join(APP_DIR_NAME).join("cache"))
}

#[cfg(target_os = "macos")]
fn platform_cache_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join("Library/Caches").join(APP_DIR_NAME))
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn platform_cache_dir() -> Option<PathBuf> {
    env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
        .map(|base| base.join(UNIX_APP_DIR_NAME))
}

#[cfg(target_os = "windows")]
fn platform_logs_dir() -> Option<PathBuf> {
    env::var_os("LOCALAPPDATA").map(|base| PathBuf::from(base).join(APP_DIR_NAME).join("logs"))
}

#[cfg(target_os = "macos")]
fn platform_logs_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from).map(|home| home.join("Library/Logs").join(APP_DIR_NAME))
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn platform_logs_dir() -> Option<PathBuf> {
    env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/state")))
        .map(|base| base.join(UNIX_APP_DIR_NAME).join("logs"))
}

fn join_checked(root: &Path, relative: &str) -> Result<PathBuf> {
    let mut path = root.to_path_buf();
    for component in Path::new(relative).components() {
        match component {
            Component::Normal(part) => path.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                bail!("path reference must stay under its root: {relative}");
            }
        }
    }
    Ok(path)
}

fn strip_first_component(path: &Path, expected: &str) -> Option<PathBuf> {
    let mut components = path.components();
    match components.next()? {
        Component::Normal(first) if first == std::ffi::OsStr::new(expected) => {
            Some(components.as_path().to_path_buf())
        }
        _ => None,
    }
}

fn path_to_slash(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_library_path_removes_windows_extended_prefixes() {
        assert_eq!(normalize_library_path(r"\\?\C:\Users\player\songs"), "C:/Users/player/songs");
        assert_eq!(normalize_library_path("//?/C:/Users/player/songs"), "C:/Users/player/songs");
        assert_eq!(normalize_library_path(r"\\?\UNC\server\share\songs"), "//server/share/songs");
    }

    fn temporary_path_root(label: &str) -> PathBuf {
        let stamp =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        env::temp_dir().join(format!("bmz-player-paths-{label}-{}-{stamp}", std::process::id()))
    }

    fn create_development_workspace(root: &Path) -> PathBuf {
        let manifest_dir = root.join("crates/bmz-player");
        std::fs::create_dir_all(root.join("target/release")).unwrap();
        std::fs::create_dir_all(root.join("data")).unwrap();
        std::fs::create_dir_all(&manifest_dir).unwrap();
        std::fs::write(root.join("Cargo.toml"), b"[workspace]\n").unwrap();
        std::fs::write(manifest_dir.join("Cargo.toml"), b"[package]\nname = \"bmz-player\"\n")
            .unwrap();
        manifest_dir
    }

    #[test]
    fn development_workspace_data_is_used_for_its_release_binary() {
        let root = temporary_path_root("development-release");
        let manifest_dir = create_development_workspace(&root);
        let exe_path = root.join("target/release/bmz-player.exe");
        std::fs::write(&exe_path, b"").unwrap();

        let resolved = development_workspace_data_dir_from(&exe_path, &manifest_dir, "bmz-player");

        assert_eq!(resolved, Some(root.join("data")));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn development_workspace_data_is_used_for_its_cargo_test_binary() {
        let root = temporary_path_root("development-test");
        let manifest_dir = create_development_workspace(&root);
        let exe_path = root.join("target/release/deps/bmz_player-a1b2c3.exe");
        std::fs::create_dir_all(exe_path.parent().unwrap()).unwrap();
        std::fs::write(&exe_path, b"").unwrap();

        let resolved = development_workspace_data_dir_from(&exe_path, &manifest_dir, "bmz-player");

        assert_eq!(resolved, Some(root.join("data")));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn copied_release_binary_does_not_reuse_build_workspace_data() {
        let root = temporary_path_root("copied-release");
        let manifest_dir = create_development_workspace(&root);
        let copied_exe = root.join("copied/target/release/bmz-player.exe");
        std::fs::create_dir_all(copied_exe.parent().unwrap()).unwrap();
        std::fs::write(&copied_exe, b"").unwrap();

        assert_eq!(
            development_workspace_data_dir_from(&copied_exe, &manifest_dir, "bmz-player"),
            None
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn development_data_precedes_current_directory_but_not_portable_data() {
        let root = temporary_path_root("development-priority");
        let current_dir = root.join("working");
        let exe_dir = root.join("target/release");
        let development_data = root.join("data");
        std::fs::create_dir_all(current_dir.join("data")).unwrap();
        std::fs::create_dir_all(exe_dir.join("data")).unwrap();
        std::fs::create_dir_all(&development_data).unwrap();

        let portable = default_data_dir(&current_dir, Some(&exe_dir), Some(&development_data));
        assert_eq!(portable.path, exe_dir.join("data"));
        assert!(portable.keep_auxiliary_dirs_with_data);

        let development = default_data_dir(
            &current_dir,
            Some(&root.join("without-data")),
            Some(&development_data),
        );
        assert_eq!(development.path, development_data);
        assert!(development.keep_auxiliary_dirs_with_data);

        let _ = std::fs::remove_dir_all(&root);
    }

    fn test_app_paths() -> AppPaths {
        AppPaths::from_dirs(
            PathBuf::from("resources"),
            PathBuf::from("data"),
            PathBuf::from("data/cache"),
            PathBuf::from("data/logs"),
        )
    }

    #[test]
    fn profile_paths_are_rooted_under_profiles_dir() {
        let app = test_app_paths();

        let paths = resolve_profile_paths(&app, "default-1").unwrap();

        assert_eq!(paths.root_dir, PathBuf::from("data/profiles/default-1"));
        assert_eq!(paths.collection_db, PathBuf::from("data/profiles/default-1/collection.db"));
        assert_eq!(paths.score_db, PathBuf::from("data/profiles/default-1/score.db"));
        assert_eq!(paths.network_db, PathBuf::from("data/profiles/default-1/network.db"));
    }

    #[test]
    fn profile_id_rejects_path_traversal() {
        assert!(validate_profile_id("../default").is_err());
        assert!(validate_profile_id("profile/name").is_err());
        assert!(validate_profile_id("").is_err());
        assert!(validate_profile_id("default_1-2").is_ok());
    }

    #[test]
    fn ensure_dirs_creates_user_skin_root() {
        let stamp =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir()
            .join(format!("bmz-player-paths-ensure-dirs-{}-{stamp}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let app = AppPaths::from_dirs(
            root.join("resources"),
            root.join("data"),
            root.join("cache"),
            root.join("logs"),
        );

        app.ensure_dirs().unwrap();

        assert!(root.join("data/skins").is_dir());

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn path_refs_resolve_against_resource_and_data_roots() {
        let app = test_app_paths();

        assert_eq!(
            app.resolve_path_ref("resource:skins/Rmz-skin/play7main.luaskin").unwrap(),
            PathBuf::from("resources/skins/Rmz-skin/play7main.luaskin")
        );
        assert_eq!(
            app.resolve_path_ref("data:skins/custom/play7.luaskin").unwrap(),
            PathBuf::from("data/skins/custom/play7.luaskin")
        );
        assert_eq!(
            app.resolve_path_ref("data/skins/legacy/play7.luaskin").unwrap(),
            PathBuf::from("data/skins/legacy/play7.luaskin")
        );
    }

    #[test]
    fn bundled_noto_cjk_font_root_is_under_resource_dir() {
        let app = test_app_paths();

        assert_eq!(app.bundled_noto_cjk_font_root(), PathBuf::from("resources/fonts/noto-cjk"));
    }

    #[test]
    fn skin_library_roots_prioritize_user_data_and_deduplicate_shared_layouts() {
        let app = test_app_paths();
        assert_eq!(
            app.skin_library_roots(),
            vec![PathBuf::from("data/skins"), PathBuf::from("resources/skins")]
        );

        let shared = AppPaths::from_dirs(
            PathBuf::from("data"),
            PathBuf::from("data"),
            PathBuf::from("data/cache"),
            PathBuf::from("data/logs"),
        );
        assert_eq!(shared.skin_library_roots(), vec![PathBuf::from("data/skins")]);
    }

    #[test]
    fn bundled_skin_label_is_hidden_for_shared_development_skin_root() {
        let app = AppPaths::from_dirs(
            PathBuf::from("data"),
            PathBuf::from("data"),
            PathBuf::from("data/cache"),
            PathBuf::from("data/logs"),
        );

        assert!(app.hides_bundled_skin_label());
    }

    #[test]
    fn bundled_skin_label_is_hidden_for_portable_sibling_data_layout() {
        let root = PathBuf::from("portable");
        let app = AppPaths::from_dirs(
            root.join("resources"),
            root.join("data"),
            root.join("data/cache"),
            root.join("data/logs"),
        );

        assert!(app.hides_bundled_skin_label());
    }

    #[test]
    fn bundled_skin_label_is_kept_for_separate_user_data_layout() {
        let root = PathBuf::from("installed");
        let app = AppPaths::from_dirs(
            root.join("resources"),
            PathBuf::from("profile-data"),
            PathBuf::from("profile-data/cache"),
            PathBuf::from("profile-data/logs"),
        );

        assert!(!app.hides_bundled_skin_label());
    }

    #[test]
    fn legacy_data_skin_paths_fall_back_to_bundled_skin_when_user_copy_is_missing() {
        let root =
            env::temp_dir().join(format!("bmz-player-paths-legacy-skin-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let resource_skin = root.join("resources/skins/Rmz-skin/play7main.luaskin");
        std::fs::create_dir_all(resource_skin.parent().unwrap()).unwrap();
        std::fs::write(&resource_skin, b"return {}").unwrap();
        let app = AppPaths::from_dirs(
            root.join("resources"),
            root.join("data"),
            root.join("cache"),
            root.join("logs"),
        );

        assert_eq!(
            app.resolve_path_ref("data/skins/Rmz-skin/play7main.luaskin").unwrap(),
            resource_skin
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn path_refs_reject_root_escape() {
        let app = test_app_paths();

        assert!(app.resolve_path_ref("resource:../profile.toml").is_err());
        assert!(app.resolve_path_ref("data:/absolute").is_err());
    }
}
