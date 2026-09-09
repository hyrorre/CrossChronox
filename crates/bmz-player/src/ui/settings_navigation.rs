use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(super) enum SettingsPage {
    #[default]
    General,
    Profile,
    Audio,
    Video,
    Input,
    Play,
    Select,
    Skin,
    Library,
    Integration,
    Import,
    Tables,
    Ir,
    Licenses,
}

impl SettingsPage {
    const ALL: [Self; 14] = [
        Self::General,
        Self::Profile,
        Self::Audio,
        Self::Video,
        Self::Input,
        Self::Play,
        Self::Select,
        Self::Skin,
        Self::Library,
        Self::Tables,
        Self::Ir,
        Self::Integration,
        Self::Import,
        Self::Licenses,
    ];

    fn label(self, text: Localizer) -> String {
        text.text(match self {
            Self::General => "settings-nav-general",
            Self::Profile => "settings-nav-profile",
            Self::Audio => "settings-nav-audio",
            Self::Video => "settings-nav-video",
            Self::Input => "settings-nav-input",
            Self::Play => "settings-nav-play",
            Self::Select => "settings-nav-select",
            Self::Skin => "settings-nav-skin",
            Self::Library => "settings-nav-library",
            Self::Integration => "settings-nav-integration",
            Self::Import => "settings-nav-import",
            Self::Tables => "settings-tables-title",
            Self::Ir => "profile-ir-title",
            Self::Licenses => "menu-licenses",
        })
    }

    fn subpages(self) -> &'static [&'static str] {
        match self {
            Self::Integration => {
                &["settings-nav-discord", "settings-nav-obs", "settings-screenshot-title"]
            }
            Self::Input => &["settings-input-title", "profile-key-config-title"],
            Self::Import => &["settings-score-import-title", "settings-nav-replay-import"],
            _ => &[],
        }
    }
}

#[derive(Clone, Default)]
pub(super) struct SettingsNavigation {
    pub(super) page: SettingsPage,
    subpages: [usize; 14],
}

#[derive(Clone, Default)]
pub(super) struct SettingsFeedback {
    dirty: std::collections::HashMap<SettingsPage, (bool, bool)>,
    error: Option<String>,
    saved: bool,
}

impl SettingsFeedback {
    fn load(ctx: &egui::Context) -> Self {
        ctx.data_mut(|data| data.get_temp(egui::Id::new("settings_feedback")).unwrap_or_default())
    }

    fn store(&self, ctx: &egui::Context) {
        ctx.data_mut(|data| data.insert_temp(egui::Id::new("settings_feedback"), self.clone()));
    }

    pub(super) fn changed(ctx: &egui::Context, app: bool, profile: bool) {
        if !app && !profile {
            return;
        }
        let mut feedback = Self::load(ctx);
        let dirty = feedback.dirty.entry(SettingsNavigation::load(ctx).page).or_default();
        dirty.0 |= app;
        dirty.1 |= profile;
        feedback.saved = false;
        feedback.store(ctx);
    }

    fn has_changes(&self, page: SettingsPage) -> bool {
        self.dirty.get(&page).is_some_and(|&(app, profile)| app || profile)
    }

    fn finish_save(&mut self, app: bool, result: Result<(), String>) {
        match result {
            Ok(()) => {
                for dirty in self.dirty.values_mut() {
                    if app {
                        dirty.0 = false;
                    } else {
                        dirty.1 = false;
                    }
                }
                self.saved = true;
            }
            Err(error) => self.error = Some(error),
        }
    }

    pub(super) fn profile_changed(ctx: &egui::Context, profile_id: &str) {
        let id = egui::Id::new("settings_feedback_profile");
        let changed = ctx.data_mut(|data| {
            let previous = data.get_temp::<String>(id);
            data.insert_temp(id, profile_id.to_owned());
            previous.as_deref() != Some(profile_id)
        });
        if changed {
            Self::default().store(ctx);
        }
    }
}

impl EguiLayer {
    /// 保存に成功した保存先の変更マークだけを消す。失敗時は変更状態を維持する。
    pub(crate) fn settings_save_finished(&mut self, app: bool, result: Result<(), String>) {
        let mut feedback = SettingsFeedback::load(&self.ctx);
        feedback.finish_save(app, result);
        feedback.store(&self.ctx);
    }
}

impl SettingsNavigation {
    pub(super) fn load(ctx: &egui::Context) -> Self {
        ctx.data_mut(|data| data.get_temp(egui::Id::new("settings_navigation")).unwrap_or_default())
    }

    pub(super) fn store(&self, ctx: &egui::Context) {
        ctx.data_mut(|data| data.insert_temp(egui::Id::new("settings_navigation"), self.clone()));
    }

    pub(super) fn select(ctx: &egui::Context, page: SettingsPage) {
        let mut state = Self::load(ctx);
        state.page = page;
        state.store(ctx);
    }

    fn subpage(&self) -> usize {
        self.subpages[self.page as usize]
    }

    pub(super) fn accepts_key_capture(&self) -> bool {
        self.page == SettingsPage::Input && self.subpage() == 1
    }
}

/// 設定セクションを選択中のページにだけ描画する。非表示のセクションは実行しない。
/// ID は翻訳された見出しに依存せず、既存の widget namespace を維持する。
pub(super) struct SettingsSection {
    page: SettingsPage,
    subpage: usize,
    title: String,
    scope: String,
    id: egui::Id,
}

impl SettingsSection {
    pub(super) fn new(page: SettingsPage, title: impl Into<String>) -> Self {
        Self {
            page,
            subpage: 0,
            title: title.into(),
            scope: String::new(),
            id: egui::Id::new(page),
        }
    }

    pub(super) fn scope(mut self, scope: String) -> Self {
        self.scope = scope;
        self
    }

    pub(super) fn id_salt(mut self, id: impl std::hash::Hash) -> Self {
        self.id = egui::Id::new(id);
        self
    }

    pub(super) fn subpage(mut self, subpage: usize) -> Self {
        self.subpage = subpage;
        self
    }

    pub(super) fn show(&self, ui: &mut egui::Ui, contents: impl FnOnce(&mut egui::Ui)) {
        let state = SettingsNavigation::load(ui.ctx());
        if state.page != self.page || state.subpage() != self.subpage {
            return;
        }
        ui.push_id(self.id, |ui| {
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.strong(&self.title);
                ui.weak(&self.scope);
            });
            ui.separator();
            contents(ui);
            ui.add_space(12.0);
        });
    }
}

/// 保存ボタンとナビゲーションをスクロール領域の外に配置する。
pub(super) fn build_settings_window(
    ctx: &egui::Context,
    open: &mut bool,
    profile_name: &str,
    text: Localizer,
    contents: impl FnOnce(&mut egui::Ui),
) -> bool {
    let mut save = false;
    localized_sized_panel_window(
        "settings_workspace",
        tr!(text, "settings-workspace-title"),
        ctx,
        open,
        900.0,
        650.0,
        egui::pos2(220.0, 32.0),
    )
    .collapsible(false)
    .show(ctx, |ui| {
        ui.label(format!("{}: {profile_name}", tr!(text, "settings-nav-profile")));
        ui.separator();
        let mut navigation = SettingsNavigation::load(ctx);
        let body_height = (ui.available_height() - 60.0).max(40.0);
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), body_height),
            egui::Layout::left_to_right(egui::Align::Min),
            |ui| {
                // 狭い画面ではカテゴリ選択を本文上部の ComboBox に移す。
                let compact = ui.available_width() < 640.0;
                if !compact {
                    ui.allocate_ui_with_layout(
                        egui::vec2(170.0, body_height),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            egui::ScrollArea::vertical()
                                .id_salt("settings_sidebar")
                                .max_height((body_height - 36.0).max(0.0))
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    ui.set_width(160.0);
                                    for page in SettingsPage::ALL {
                                        if page == SettingsPage::Licenses {
                                            continue;
                                        }
                                        let label = if SettingsFeedback::load(ctx).has_changes(page)
                                        {
                                            format!("{} •", page.label(text))
                                        } else {
                                            page.label(text)
                                        };
                                        ui.selectable_value(&mut navigation.page, page, label);
                                    }
                                });
                            ui.separator();
                            ui.selectable_value(
                                &mut navigation.page,
                                SettingsPage::Licenses,
                                SettingsPage::Licenses.label(text),
                            );
                        },
                    );
                    ui.separator();
                }
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), body_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        if compact {
                            egui::ComboBox::from_id_salt("settings_category")
                                .selected_text(navigation.page.label(text))
                                .show_ui(ui, |ui| {
                                    for page in SettingsPage::ALL {
                                        ui.selectable_value(
                                            &mut navigation.page,
                                            page,
                                            page.label(text),
                                        );
                                    }
                                });
                        } else {
                            ui.heading(navigation.page.label(text));
                        }
                        let subpages = navigation.page.subpages();
                        if !subpages.is_empty() {
                            let subpage = &mut navigation.subpages[navigation.page as usize];
                            egui::ComboBox::from_id_salt(("settings_subpage", navigation.page))
                                .selected_text(text.text(subpages[*subpage]))
                                .show_ui(ui, |ui| {
                                    for (index, key) in subpages.iter().enumerate() {
                                        ui.selectable_value(subpage, index, text.text(key));
                                    }
                                });
                        }
                        navigation.store(ctx);
                        if navigation.page == SettingsPage::Skin && ui.available_height() >= 380.0 {
                            contents(ui);
                        } else {
                            egui::ScrollArea::vertical()
                                .id_salt((
                                    "settings_content",
                                    navigation.page,
                                    navigation.subpage(),
                                ))
                                .auto_shrink([false, false])
                                .max_height(ui.available_height())
                                .show(ui, contents);
                        }
                    },
                );
            },
        );
        ui.separator();
        ui.horizontal_wrapped(|ui| {
            save = ui.button(tr!(text, "settings-save-all")).clicked();
            let mut feedback = SettingsFeedback::load(ctx);
            if save {
                feedback.error = None;
                feedback.store(ctx);
            }
            if let Some(error) = &feedback.error {
                ui.colored_label(egui::Color32::LIGHT_RED, tr!(text, "settings-save-failed"))
                    .on_hover_text(error);
            } else if feedback.dirty.values().any(|&(app, profile)| app || profile) {
                ui.label(tr!(text, "settings-unsaved"));
            } else if feedback.saved {
                ui.label(tr!(text, "settings-saved"));
            } else {
                ui.small(tr!(text, "settings-save-all-help"));
            }
        });
    });
    save
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_save_keeps_changes_when_the_other_config_saves_successfully() {
        let mut feedback = SettingsFeedback::default();
        feedback.dirty.insert(SettingsPage::Audio, (true, true));
        feedback.finish_save(true, Err("Disk full".into()));
        feedback.finish_save(false, Ok(()));
        assert_eq!(feedback.dirty[&SettingsPage::Audio], (true, false));
        assert_eq!(feedback.error.as_deref(), Some("Disk full"));
        assert!(feedback.has_changes(SettingsPage::Audio));
        feedback.error = None;
        feedback.finish_save(true, Ok(()));
        assert!(!feedback.has_changes(SettingsPage::Audio));
    }

    #[test]
    fn hidden_sections_do_not_run_and_subpage_selection_survives_navigation() {
        let ctx = egui::Context::default();
        let mut navigation = SettingsNavigation::default();
        navigation.page = SettingsPage::Input;
        navigation.subpages[SettingsPage::Input as usize] = 1;
        navigation.store(&ctx);
        assert!(navigation.accepts_key_capture());
        SettingsNavigation::select(&ctx, SettingsPage::Audio);
        assert!(!SettingsNavigation::load(&ctx).accepts_key_capture());
        SettingsNavigation::select(&ctx, SettingsPage::Input);
        assert!(SettingsNavigation::load(&ctx).accepts_key_capture());
        let mut visible = false;
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            SettingsSection::new(SettingsPage::Audio, "hidden")
                .show(ui, |_| panic!("hidden section ran"));
            SettingsSection::new(SettingsPage::Input, "devices")
                .show(ui, |_| panic!("wrong subpage ran"));
            SettingsSection::new(SettingsPage::Input, "bindings")
                .subpage(1)
                .show(ui, |_| visible = true);
        });
        assert!(visible);
    }

    #[test]
    fn settings_window_keeps_save_button_inside_viewport_with_long_content() {
        for size in [egui::vec2(480.0, 480.0), egui::vec2(1280.0, 800.0)] {
            let ctx = egui::Context::default();
            let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, size);
            let mut saw_save_button = false;
            for _ in 0..3 {
                let output = ctx.run_ui(
                    egui::RawInput { screen_rect: Some(rect), ..Default::default() },
                    |ui| {
                        build_settings_window(
                            ui.ctx(),
                            &mut true,
                            "Test",
                            Localizer::new(AppLocale::En),
                            |ui| {
                                for _ in 0..100 {
                                    ui.label("A setting with explanatory text");
                                }
                            },
                        );
                    },
                );
                let save = output.shapes.iter().find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text == "Save all changes" => {
                        Some((shape.clip_rect, text.galley.rect.translate(text.pos.to_vec2())))
                    }
                    _ => None,
                });
                // Window の初回 sizing pass には描画がないことがある。
                if let Some((clip, button)) = save {
                    saw_save_button = true;
                    assert!(rect.contains_rect(button), "save button outside viewport: {button:?}");
                    assert!(clip.contains_rect(button), "save button clipped: {button:?}");
                }
            }
            assert!(saw_save_button, "save button was not rendered");
            let window =
                ctx.memory(|memory| memory.area_rect(egui::Id::new("settings_workspace"))).unwrap();
            assert!(rect.contains_rect(window), "window outside viewport: {window:?}");
        }
    }

    #[test]
    fn license_navigation_stays_at_the_bottom_of_the_sidebar() {
        let ctx = egui::Context::default();
        let text = Localizer::new(AppLocale::En);
        let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0));
        let mut license_rect = None;
        let mut save_rect = None;
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput { screen_rect: Some(rect), ..Default::default() },
                |ui| {
                    build_settings_window(ui.ctx(), &mut true, "Default", text, |_| {});
                },
            );
            for shape in output.shapes {
                if let egui::Shape::Text(label) = shape.shape {
                    let rect = label.galley.rect.translate(label.pos.to_vec2());
                    if label.galley.job.text == text.text("menu-licenses") {
                        license_rect = Some(rect);
                    }
                    if label.galley.job.text == text.text("settings-save-all") {
                        save_rect = Some(rect);
                    }
                }
            }
        }
        let license = license_rect.expect("license navigation is visible");
        let save = save_rect.expect("save button is visible");
        assert!(license.top() > 400.0, "license should be pinned below the category list");
        assert!(license.bottom() < save.top(), "license should be above the footer");
    }
}
