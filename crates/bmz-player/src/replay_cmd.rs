use anyhow::{Context, Result};
use bmz_core::input::InputDeviceKind;

use crate::cli::ReplayCommand;
use crate::config::app_config::AppConfig;
use crate::config::load::load_app_config;
use crate::paths::{AppPaths, resolve_app_paths, resolve_profile_paths};
use crate::storage::library_db::LibraryDatabase;
use crate::storage::migration::{migrate_library_db, migrate_score_db};
use crate::storage::replay_import::write_replay_import_details;
use crate::storage::replay_import::{ImportBeatorajaReplaysRequest, import_beatoraja_replays};
use crate::storage::score_db::ScoreDatabase;

pub fn run_replay_command(command: ReplayCommand) -> Result<()> {
    let app_paths = resolve_app_paths()?;
    run_replay_command_with_paths(command, &app_paths)
}

pub fn run_replay_command_with_paths(command: ReplayCommand, app_paths: &AppPaths) -> Result<()> {
    run_replay_command_with_paths_and_profile(command, app_paths, None)
}

pub fn run_replay_command_with_paths_and_profile(
    command: ReplayCommand,
    app_paths: &AppPaths,
    profile_id: Option<&str>,
) -> Result<()> {
    match command {
        ReplayCommand::Import { path, overwrite, controller } => {
            import_replays(app_paths, profile_id, &path, overwrite, controller)
        }
    }
}

fn import_replays(
    app_paths: &AppPaths,
    profile_id: Option<&str>,
    path: &str,
    overwrite: bool,
    controller: bool,
) -> Result<()> {
    app_paths.ensure_dirs()?;
    let app_config = if app_paths.config_toml.exists() {
        load_app_config(&app_paths.config_toml)
            .with_context(|| format!("failed to load {}", app_paths.config_toml.display()))?
    } else {
        AppConfig::default()
    };
    let profile_override = profile_id;
    let profile_id = profile_override.unwrap_or(&app_config.active_profile);
    let profile_paths = resolve_profile_paths(app_paths, profile_id)?;
    if profile_override.is_some() && !profile_paths.profile_toml.is_file() {
        anyhow::bail!("profile not found: {profile_id}");
    }
    profile_paths.ensure_dirs()?;
    migrate_library_db(&app_paths.library_db)?;
    migrate_score_db(&profile_paths.score_db)?;
    let library_db = LibraryDatabase::open(&app_paths.library_db)?;
    let mut score_db = ScoreDatabase::open(&profile_paths.score_db)?;
    let mut request = ImportBeatorajaReplaysRequest::new(path);
    request.overwrite_protected_slots = overwrite;
    request.device_kind =
        if controller { InputDeviceKind::Controller } else { InputDeviceKind::Keyboard };
    let mut report =
        import_beatoraja_replays(&library_db, &mut score_db, &profile_paths, &request)?;
    if !report.issues.is_empty() {
        let details = write_replay_import_details(&app_paths.logs_dir, &request.source, &report)?;
        report.details_path = Some(details);
    }

    println!("{}", report.summary());
    if let Some(warning) = &report.threshold_warning {
        println!("warning: {warning}");
    }
    for issue in &report.issues {
        println!("{}: {}", issue.path.display(), issue.message);
    }
    if let Some(path) = &report.details_path {
        println!("details: {}", path.display());
    }
    Ok(())
}
