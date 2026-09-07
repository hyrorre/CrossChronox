use std::collections::BTreeSet;
use std::io::Write as _;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::cli::{IrCommand, RivalAction};
use crate::config::load::{load_app_config, load_profile_config};
use crate::config::profile_config::{
    IrConfig, IrProviderConfig, IrProviderRoleConfig, IrSendPolicyConfig, ProfileConfig,
};
use crate::config::save::save_profile_config;
use crate::ir::backfill::{
    IrLocalUploadOptions, enqueue_local_score_jobs, resolve_local_upload_target,
};
use crate::ir::bmz_official::{BmzOfficialIrClient, IrRankingRequest};
use crate::ir::credentials::{
    IrStoredCredentials, delete_credentials, load_credentials, save_credentials,
};
use crate::ir::download::{IrScoreDownloadOptions, download_ir_scores};
use crate::ir::sync::{
    IR_CLI_SYNC_BATCH_LIMIT, IR_CLI_SYNC_JOB_SPACING_MS, IrSyncJobFilter, IrSyncReport,
    IrSyncThrottle, ensure_fresh_credentials, sync_pending_ir_jobs, sync_pending_ir_jobs_filtered,
};
use crate::ir::types::IrRankingScope;
use crate::paths::{AppPaths, ProfilePaths, resolve_app_paths, resolve_profile_paths};
use crate::storage::library_db::LibraryDatabase;
use crate::storage::network_db::{IrJobKind, NetworkDatabase};
use crate::storage::score_db::ScoreDatabase;

mod account;
mod auth;
mod cleanup;
mod dispatch;
mod download;
mod jobs;
mod ranking;
mod upload;

pub use account::sync_ir_rivals_into_profile;
pub use dispatch::{
    run_ir_command, run_ir_command_with_paths, run_ir_command_with_paths_and_profile,
};
#[cfg(test)]
use upload::ensure_full_upload_progress;

#[cfg(test)]
#[path = "ir_cmd/tests.rs"]
mod tests;
