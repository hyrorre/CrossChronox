use super::*;

pub(in crate::ui) fn build_profile_settings_panel(
    ui: &mut egui::Ui,
    context: ProfileSettingsPanelContext<'_>,
) -> ProfileSettingsPanelActions {
    let ProfileSettingsPanelContext {
        profile,
        app_config,
        show_fps,
        ir_login,
        ir_device_key,
        profile_manager,
        key_config,
        profile_root,
        unrestricted,
        text,
    } = context;

    if !SettingsNavigation::load(ui.ctx()).accepts_key_capture() || !unrestricted {
        key_config.listening = None;
    }

    // 非同期ログインの完了は runtime 側で、表示ページに関係なく反映する。
    let save_clicked = false;
    let readonly_profile = (!unrestricted).then(|| profile.clone());
    let readonly_app_config = (!unrestricted).then(|| app_config.clone());
    let mut section = ProfileSectionContext {
        profile,
        app_config,
        show_fps,
        ir_login,
        ir_device_key,
        profile_manager,
        key_config,
        profile_root,
        unrestricted,
        text,
        save_clicked,
        save_app_config: false,
        key_config_action: None,
    };

    if !section.unrestricted {
        ui.label(tr!(section.text, "profile-settings-restricted"));
        ui.separator();
    }
    build_profile_basic_section(ui, &mut section);
    section.save_app_config |= build_profile_manager_section(
        ui,
        section.app_config,
        section.profile,
        section.profile_manager,
        section.unrestricted,
        section.text,
    );
    build_profile_volume_section(ui, &mut section);
    build_profile_judge_section(ui, &mut section);
    build_profile_play_section(ui, &mut section);
    build_profile_display_section(ui, &mut section);
    build_profile_select_section(ui, &mut section);
    build_profile_input_section(ui, &mut section);
    build_profile_key_config_section(ui, &mut section);
    build_profile_replay_section(ui, &mut section);
    build_profile_system_sound_section(ui, &mut section);
    build_profile_ir_section(ui, &mut section);
    build_profile_ui_section(ui, &mut section);

    if let Some(readonly) = readonly_profile {
        restore_restricted_profile_settings(section.profile, readonly);
    }
    if let Some(readonly) = readonly_app_config {
        *section.app_config = readonly;
        section.save_app_config = false;
    }
    ProfileSettingsPanelActions {
        save: section.save_clicked,
        save_app_config: section.save_app_config,
        key_config_action: section.key_config_action,
    }
}
