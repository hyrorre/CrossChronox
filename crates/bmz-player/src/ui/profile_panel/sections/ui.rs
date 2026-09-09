use super::*;

pub(in crate::ui::profile_panel) fn build_profile_ui_section(
    ui: &mut egui::Ui,
    section: &mut ProfileSectionContext<'_>,
) {
    let profile = &mut *section.profile;
    let show_fps = &mut *section.show_fps;
    let unrestricted = section.unrestricted;
    let mut text = section.text;
    SettingsSection::new(SettingsPage::General, tr!(text, "profile-ui-title"))
        .scope(tr!(text, "settings-scope-profile"))
        .id_salt("profile_ui")
        .show(ui, |ui| {
            if !unrestricted {
                ui.disable();
            }
            let current_locale = profile.ui.locale();
            let mut selected_locale = current_locale;
            egui::ComboBox::new("profile_ui_language", tr!(text, "profile-ui-language"))
                .selected_text(selected_locale.native_name())
                .show_ui(ui, |ui| {
                    for locale in AppLocale::SUPPORTED {
                        ui.selectable_value(&mut selected_locale, locale, locale.native_name());
                    }
                });
            if selected_locale != current_locale {
                profile.ui.set_locale(selected_locale);
                text = Localizer::new(selected_locale);
                section.save_clicked = true;
            }
            ui.horizontal(|ui| {
                ui.label(tr!(text, "profile-ui-theme-unimplemented"));
                ui.text_edit_singleline(&mut profile.ui.theme);
            });
            if ui.checkbox(show_fps, tr!(text, "settings-show-fps")).changed() {
                profile.ui.show_fps = *show_fps;
            }
            ui.checkbox(
                &mut profile.ui.confirm_on_exit,
                tr!(text, "profile-ui-confirm-exit-unimplemented"),
            );
        });
    section.text = text;
}
