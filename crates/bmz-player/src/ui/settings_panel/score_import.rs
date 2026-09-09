use super::*;

pub(in crate::ui) fn build_score_import_section(
    ui: &mut egui::Ui,
    path: &mut String,
    kind: &mut ScoreImportKind,
    device_type: &mut InputDeviceKind,
    status: &str,
    error: &str,
    request: &mut Option<ScoreImportRequest>,
    text: Localizer,
) {
    SettingsSection::new(SettingsPage::Import, tr!(text, "settings-score-import-title"))
        .scope(tr!(text, "settings-scope-profile"))
        .id_salt("settings_score_import")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("DB");
                ui.add(
                    egui::TextEdit::singleline(path)
                        .desired_width(260.0)
                        .hint_text("score.db / scoredatalog.db / LR2 score db"),
                );
            });
            ui.horizontal(|ui| {
                if ui.button(tr!(text, "common-choose-file")).clicked()
                    && let Some(file) =
                        rfd::FileDialog::new().add_filter("SQLite DB", &["db"]).pick_file()
                {
                    *path = file.to_string_lossy().into_owned();
                }
                egui::ComboBox::from_id_salt("score_import_kind")
                    .selected_text(kind.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            kind,
                            ScoreImportKind::Lr2,
                            ScoreImportKind::Lr2.label(),
                        );
                        ui.selectable_value(
                            kind,
                            ScoreImportKind::Beatoraja,
                            ScoreImportKind::Beatoraja.label(),
                        );
                        ui.selectable_value(
                            kind,
                            ScoreImportKind::Lr2Oraja,
                            ScoreImportKind::Lr2Oraja.label(),
                        );
                        ui.selectable_value(
                            kind,
                            ScoreImportKind::Lr2OrajaDx,
                            ScoreImportKind::Lr2OrajaDx.label(),
                        );
                    });
            });
            ui.horizontal(|ui| {
                ui.label(tr!(text, "settings-score-import-device"));
                ui.selectable_value(
                    device_type,
                    InputDeviceKind::Keyboard,
                    tr!(text, "settings-input-keyboard"),
                );
                ui.selectable_value(
                    device_type,
                    InputDeviceKind::Controller,
                    tr!(text, "settings-score-import-controller"),
                );
            });
            if ui.button(tr!(text, "settings-score-import-button")).clicked() {
                let trimmed = path.trim();
                if trimmed.is_empty() {
                    *request = None;
                } else {
                    *request = Some(ScoreImportRequest {
                        path: PathBuf::from(trimmed),
                        kind: *kind,
                        device_type: *device_type,
                    });
                }
            }
            if !status.is_empty() {
                ui.colored_label(egui::Color32::LIGHT_GREEN, status);
            }
            if !error.is_empty() {
                ui.colored_label(egui::Color32::RED, error);
            }
        });
}

pub(in crate::ui) fn build_replay_import_section(
    ui: &mut egui::Ui,
    path: &mut String,
    device_type: &mut InputDeviceKind,
    overwrite: &mut bool,
    status: &str,
    error: &str,
    progress: Option<ReplayImportProgress>,
    request: &mut Option<ImportBeatorajaReplaysRequest>,
    cancel: &mut bool,
    text: Localizer,
) {
    SettingsSection::new(SettingsPage::Import, "beatoraja Replay Import (.brd)")
        .scope(tr!(text, "settings-scope-profile"))
        .subpage(1)
        .id_salt("settings_replay_import")
        .show(ui, |ui| {
            ui.label("playerフォルダ、replayフォルダ、または単一の.brdファイルを指定します。");
            ui.horizontal(|ui| {
                ui.label("PATH");
                ui.add(
                    egui::TextEdit::singleline(path)
                        .desired_width(260.0)
                        .hint_text("/path/to/player or replay"),
                );
            });
            ui.horizontal(|ui| {
                if ui.button("フォルダを選択").clicked()
                    && let Some(folder) = rfd::FileDialog::new().pick_folder()
                {
                    *path = folder.to_string_lossy().into_owned();
                }
                if ui.button(".brdを選択").clicked()
                    && let Some(file) =
                        rfd::FileDialog::new().add_filter("beatoraja Replay", &["brd"]).pick_file()
                {
                    *path = file.to_string_lossy().into_owned();
                }
            });
            ui.horizontal(|ui| {
                ui.label("入力デバイス");
                ui.selectable_value(device_type, InputDeviceKind::Keyboard, "Keyboard");
                ui.selectable_value(device_type, InputDeviceKind::Controller, "Controller");
            });
            ui.checkbox(overwrite, "既存のローカルReplayスロットも上書きする");
            if ui.add_enabled(progress.is_none(), egui::Button::new("Replayをインポート")).clicked()
            {
                let trimmed = path.trim();
                if trimmed.is_empty() {
                    *request = None;
                } else {
                    let mut next = ImportBeatorajaReplaysRequest::new(trimmed);
                    next.overwrite_protected_slots = *overwrite;
                    next.device_kind = *device_type;
                    *request = Some(next);
                }
            }
            if let Some(progress) = progress {
                let fraction = if progress.total == 0 {
                    0.0
                } else {
                    progress.done as f32 / progress.total as f32
                };
                ui.add(
                    egui::ProgressBar::new(fraction)
                        .show_percentage()
                        .text(format!("{} / {}", progress.done, progress.total)),
                );
                if ui.button("インポートをキャンセル").clicked() {
                    *cancel = true;
                }
            }
            if !status.is_empty() {
                ui.colored_label(egui::Color32::LIGHT_GREEN, status);
            }
            if !error.is_empty() {
                ui.colored_label(egui::Color32::RED, error);
            }
        });
}
