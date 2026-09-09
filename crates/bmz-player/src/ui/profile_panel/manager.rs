pub(in crate::ui) fn build_profile_manager_section(
    ui: &mut egui::Ui,
    app_config: &mut AppConfig,
    profile: &ProfileConfig,
    state: &mut ProfileManagerUiState,
    editable: bool,
    text: Localizer,
) -> bool {
    let mut save_app_config = false;
    SettingsSection::new(SettingsPage::Profile, tr!(text, "profile-manager-title"))
        .scope(tr!(text, "settings-scope-app"))
        .id_salt("profile_manager")
        .show(ui, |ui| {
            if !editable {
                ui.disable();
            }
            let app_paths = match resolve_app_paths() {
                Ok(paths) => paths,
                Err(error) => {
                    ui.colored_label(egui::Color32::RED, format!("{error:#}"));
                    return;
                }
            };
            let profiles = match profile_cmd::profile_summaries(&app_paths) {
                Ok(profiles) => profiles,
                Err(error) => {
                    ui.colored_label(egui::Color32::RED, format!("{error:#}"));
                    return;
                }
            };

            if state.copy_source_id.is_empty() {
                state.copy_source_id = profile.id.clone();
            }

            ui.horizontal(|ui| {
                ui.label(tr!(text, "profile-manager-current"));
                ui.monospace(&profile.id);
            });
            ui.horizontal(|ui| {
                ui.label(tr!(text, "profile-manager-next-startup"));
                egui::ComboBox::from_id_salt("profile_active_next")
                    .selected_text(profile_selection_label(&profiles, &app_config.active_profile))
                    .show_ui(ui, |ui| {
                        let active_profile = app_config.active_profile.clone();
                        for summary in &profiles {
                            let selected = summary.id == active_profile;
                            let label = profile_selection_label(&profiles, &summary.id);
                            if ui.selectable_label(selected, label).clicked() && !selected {
                                app_config.active_profile = summary.id.clone();
                                state.message = tr!(
                                    text,
                                    "profile-manager-next-startup-changed",
                                    "id" => summary.id.clone(),
                                );
                                state.error.clear();
                                save_app_config = true;
                            }
                        }
                    });
            });

            ui.separator();
            ui.label(tr!(text, "profile-manager-create-title"));
            ui.horizontal(|ui| {
                ui.label("ID");
                profile_id_text_edit(ui, &mut state.create_id);
            });
            ui.horizontal(|ui| {
                ui.label(tr!(text, "profile-display-name"));
                ui.text_edit_singleline(&mut state.create_display_name);
            });
            ui.checkbox(&mut state.create_activate, tr!(text, "profile-manager-activate-next"));
            if ui.button(tr!(text, "profile-manager-create")).clicked() {
                let id = state.create_id.trim().to_string();
                let display_name =
                    trimmed_non_empty(&state.create_display_name).map(str::to_string);
                match profile_cmd::create_profile(&app_paths, &id, display_name.as_deref(), false) {
                    Ok(()) => {
                        if state.create_activate {
                            app_config.active_profile = id.clone();
                            save_app_config = true;
                        }
                        state.message = tr!(text, "profile-manager-created", "id" => id.clone());
                        state.error.clear();
                        state.create_id.clear();
                        state.create_display_name.clear();
                    }
                    Err(error) => {
                        state.error = format!("{error:#}");
                        state.message.clear();
                    }
                }
            }

            ui.separator();
            ui.label(tr!(text, "profile-manager-copy-title"));
            ui.horizontal(|ui| {
                ui.label(tr!(text, "profile-manager-copy-source"));
                egui::ComboBox::from_id_salt("profile_copy_source")
                    .selected_text(profile_selection_label(&profiles, &state.copy_source_id))
                    .show_ui(ui, |ui| {
                        for summary in &profiles {
                            let selected = summary.id == state.copy_source_id;
                            let label = profile_selection_label(&profiles, &summary.id);
                            if ui.selectable_label(selected, label).clicked() {
                                state.copy_source_id = summary.id.clone();
                            }
                        }
                    });
            });
            ui.horizontal(|ui| {
                ui.label(tr!(text, "profile-manager-new-id"));
                profile_id_text_edit(ui, &mut state.copy_target_id);
            });
            ui.horizontal(|ui| {
                ui.label(tr!(text, "profile-display-name"));
                ui.text_edit_singleline(&mut state.copy_display_name);
            });
            ui.checkbox(&mut state.copy_activate, tr!(text, "profile-manager-activate-next"));
            if ui.button(tr!(text, "profile-manager-copy")).clicked() {
                let source_id = state.copy_source_id.trim().to_string();
                let target_id = state.copy_target_id.trim().to_string();
                let display_name = trimmed_non_empty(&state.copy_display_name).map(str::to_string);
                match profile_cmd::copy_profile(
                    &app_paths,
                    &source_id,
                    &target_id,
                    display_name.as_deref(),
                    false,
                ) {
                    Ok(()) => {
                        if state.copy_activate {
                            app_config.active_profile = target_id.clone();
                            save_app_config = true;
                        }
                        state.message = tr!(
                            text,
                            "profile-manager-copied",
                            "source_id" => source_id,
                            "target_id" => target_id.clone(),
                        );
                        state.error.clear();
                        state.copy_target_id.clear();
                        state.copy_display_name.clear();
                    }
                    Err(error) => {
                        state.error = format!("{error:#}");
                        state.message.clear();
                    }
                }
            }

            if !state.message.is_empty() {
                ui.colored_label(egui::Color32::LIGHT_GREEN, state.message.as_str());
            }
            if !state.error.is_empty() {
                ui.colored_label(egui::Color32::RED, state.error.as_str());
            }
        });
    save_app_config
}
use super::*;
