use super::*;

#[test]
fn select_settings_screen_volume_numbers_match_beatoraja_refs() {
    let state = SkinDrawState {
        select_screen: true,
        in_settings: true,
        select_master_volume: 0.42,
        select_key_volume: 0.73,
        select_bgm_volume: 0.18,
        ..SkinDrawState::default()
    };

    assert_eq!(skin_state_number(57, &state), Some(42));
    assert_eq!(skin_state_number(58, &state), Some(73));
    assert_eq!(skin_state_number(59, &state), Some(18));
}

#[test]
fn select_option_event_indices_expose_guide_assists_and_constant() {
    let state = SkinDrawState {
        select_screen: true,
        select_difficulty_filter_index: 4,
        guide_se_enabled: true,
        assist_extra_note_depth: 3,
        assist_mine_mode: 4,
        assist_scroll_mode: 2,
        assist_long_note_mode: 5,
        constant_enabled: true,
        ..SkinDrawState::default()
    };

    assert_eq!(skin_state_event_index(10, &state), 4);
    assert_eq!(skin_state_event_index(309, &state), 4);
    assert_eq!(skin_state_event_index(343, &state), 1);
    assert_eq!(skin_state_event_index(350, &state), 3);
    assert_eq!(skin_state_event_index(351, &state), 4);
    assert_eq!(skin_state_event_index(352, &state), 2);
    assert_eq!(skin_state_event_index(353, &state), 5);
    assert_eq!(skin_state_event_index(400, &state), 1);
    assert_eq!(skin_image_index_number(343, &state), Some(1));
    assert_eq!(skin_image_index_number(400, &state), Some(1));
    assert!(test_skin_op(400, &[], &state));
}

#[test]
fn select_duration_ref_preserves_exact_configured_milliseconds() {
    let state = SkinDrawState {
        select_screen: true,
        total_duration_ms: 497,
        duration_green_ms: Some(duration_to_green_number_ms(497)),
        ..SkinDrawState::default()
    };

    assert_eq!(skin_state_number(312, &state), Some(497));
    assert_eq!(skin_state_number(313, &state), Some(298));
}

#[test]
fn select_rank_and_judge_ops_are_hidden_in_settings() {
    let state = SkinDrawState {
        select_screen: true,
        select_row_kind: SelectRowKind::Config,
        select_in_library: true,
        select_ex_score: Some(1556),
        select_total_notes: 1000,
        judge_rank: Some(2),
        in_settings: true,
        ..SkinDrawState::default()
    };

    assert!(!test_skin_op(200, &[], &state));
    assert!(!test_skin_op(201, &[], &state));
    assert!(!test_skin_op(302, &[], &state));
    assert!(!test_skin_op(180, &[], &state));
}

#[test]
fn select_detail_uses_beatoraja_fields_for_setting_description_and_editing_state() {
    let snapshot = SelectSnapshot {
        in_settings: true,
        settings_editing: true,
        selected_index: 0,
        rows: vec![SelectRowSnapshot {
            index: 0,
            title: "MASTER".to_string(),
            bar_text: "MASTER [25]".to_string(),
            subtitle: "MASTER is currently set to 25".to_string(),
            artist: "25".to_string(),
            kind: SelectRowKind::Config,
            ..SelectRowSnapshot::default()
        }],
        ..SelectSnapshot::default()
    };
    let row = &snapshot.rows[0];
    assert_eq!(row.display_bar_text(), "MASTER [25]");
    assert_eq!(select_detail_title(&snapshot, Some(row)), "MASTER");
    assert_eq!(select_detail_artist(&snapshot, Some(row)), "25");
    assert_eq!(select_detail_genre(&snapshot, Some(row)), "[編集中]");
    assert_eq!(select_detail_subtitle(&snapshot, Some(row)), "MASTER is currently set to 25");

    let song_snapshot = SelectSnapshot {
        selected_index: 0,
        rows: vec![SelectRowSnapshot {
            index: 0,
            title: "Song".to_string(),
            subtitle: "Subtitle".to_string(),
            artist: "Artist".to_string(),
            genre: "Genre".to_string(),
            kind: SelectRowKind::Song,
            ..SelectRowSnapshot::default()
        }],
        ..SelectSnapshot::default()
    };
    let song = &song_snapshot.rows[0];
    assert_eq!(select_detail_artist(&song_snapshot, Some(song)), "Artist");
    assert_eq!(select_detail_genre(&song_snapshot, Some(song)), "Genre");
    assert_eq!(select_detail_subtitle(&song_snapshot, Some(song)), "Subtitle");

    assert_eq!(
        skin_state_text(
            &SkinTextDef { id: "t".to_string(), ref_id: 3, ..SkinTextDef::default() },
            &SkinTextState { target: "", ..SkinTextState::default() },
        ),
        ""
    );
}

#[test]
fn nearest_select_diff_uses_bmz_refs_without_grade_fallback() {
    let document: SkinDocument = serde_json::from_str(
        r#"
            {
                "type": 5,
                "w": 100,
                "h": 100,
                "source": [
                    {"id": "rank", "path": "rank.png"}
                ],
                "image": [
                    {"id": "rank_f", "src": "rank", "x": 0, "y": 0, "w": 45, "h": 19},
                    {"id": "rank_e", "src": "rank", "x": 45, "y": 0, "w": 45, "h": 19}
                ],
                "imageset": [{
                    "id": "nearest_rank",
                    "ref": 1976,
                    "images": ["rank_f", "rank_e", "rank_e", "rank_e", "rank_e", "rank_e", "rank_e", "rank_e", "rank_e"]
                }],
                "value": [
                    {
                        "id": "RANK_Diff_Exscore",
                        "src": "num",
                        "x": 0,
                        "y": 0,
                        "w": 120,
                        "h": 40,
                        "divx": 12,
                        "divy": 2,
                        "digit": 4,
                        "ref": 1980,
                        "zeropadding": 2
                    }
                ],
                "destination": [
                    {
                        "id": "nearest_rank",
                        "op": [1985],
                        "dst": [{"x": 0, "y": 20, "w": 10, "h": 10}]
                    },
                    {
                        "id": "RANK_Diff_Exscore",
                        "dst": [{"x": 10, "y": 20, "w": 10, "h": 10}]
                    }
                ]
            }
            "#,
    )
    .unwrap();
    let sources = HashMap::from([
        (
            "num".to_string(),
            SkinDocumentTexture {
                source_id: "num".to_string(),
                texture: SkinTextureId(42),
                source_size: SkinImageSize { width: 120.0, height: 40.0 },
            },
        ),
        (
            "rank".to_string(),
            SkinDocumentTexture {
                source_id: "rank".to_string(),
                texture: SkinTextureId(7),
                source_size: SkinImageSize { width: 90.0, height: 19.0 },
            },
        ),
    ]);
    let snapshot = SelectSnapshot {
        rows: vec![SelectRowSnapshot {
            index: 0,
            ex_score: Some(100),
            total_notes: 1000,
            in_library: true,
            ..SelectRowSnapshot::default()
        }],
        chart_count: 1,
        ..SelectSnapshot::default()
    };

    let items = document.select_render_items(&sources, &snapshot);
    let first_digit_uv = items.iter().find_map(|item| match item {
        SkinRenderItem::Image { texture: SkinTextureId(42), uv, .. } => Some(*uv),
        _ => None,
    });

    assert_eq!(first_digit_uv.map(|uv| uv.y), Some(0.0));
    assert!(
        items
            .iter()
            .any(|item| matches!(item, SkinRenderItem::Image { texture: SkinTextureId(7), .. }))
    );
}

#[test]
fn next_select_diff_number_renders_next_rank_label() {
    let document: SkinDocument = serde_json::from_str(
        r#"
            {
                "type": 5,
                "w": 100,
                "h": 100,
                "source": [
                    {"id": "rank", "path": "rank.png"}
                ],
                "image": [
                    {"id": "rank_f", "src": "rank", "x": 0, "y": 0, "w": 45, "h": 19},
                    {"id": "rank_e", "src": "rank", "x": 45, "y": 0, "w": 45, "h": 19}
                ],
                "imageset": [{
                    "id": "next_rank",
                    "ref": 1975,
                    "images": ["rank_f", "rank_e", "rank_e", "rank_e", "rank_e", "rank_e", "rank_e", "rank_e", "rank_e"]
                }],
                "value": [
                    {
                        "id": "RANK_Diff_Exscore",
                        "src": "num",
                        "x": 0,
                        "y": 0,
                        "w": 120,
                        "h": 40,
                        "divx": 12,
                        "divy": 2,
                        "digit": 4,
                        "ref": 154,
                        "zeropadding": 2
                    }
                ],
                "destination": [
                    {
                        "id": "next_rank",
                        "op": [1985],
                        "dst": [{"x": 0, "y": 20, "w": 10, "h": 10}]
                    },
                    {
                        "id": "RANK_Diff_Exscore",
                        "dst": [{"x": 10, "y": 20, "w": 10, "h": 10}]
                    }
                ]
            }
            "#,
    )
    .unwrap();
    let sources = HashMap::from([
        (
            "num".to_string(),
            SkinDocumentTexture {
                source_id: "num".to_string(),
                texture: SkinTextureId(42),
                source_size: SkinImageSize { width: 120.0, height: 40.0 },
            },
        ),
        (
            "rank".to_string(),
            SkinDocumentTexture {
                source_id: "rank".to_string(),
                texture: SkinTextureId(7),
                source_size: SkinImageSize { width: 90.0, height: 19.0 },
            },
        ),
    ]);
    let snapshot = SelectSnapshot {
        rows: vec![SelectRowSnapshot {
            index: 0,
            ex_score: Some(0),
            play_count: 1,
            total_notes: 2253,
            in_library: true,
            ..SelectRowSnapshot::default()
        }],
        chart_count: 1,
        ..SelectSnapshot::default()
    };

    let items = document.select_render_items(&sources, &snapshot);
    let first_digit_uv = items.iter().find_map(|item| match item {
        SkinRenderItem::Image { texture: SkinTextureId(42), uv, .. } => Some(*uv),
        _ => None,
    });

    let (state, _) = document.select_draw_state(&snapshot, None);
    assert_eq!(skin_state_number(154, &state), Some(1002));
    assert_eq!(first_digit_uv.map(|uv| uv.y), Some(0.0));
    assert!(items.iter().any(|item| matches!(
        item,
        SkinRenderItem::Image {
            texture: SkinTextureId(7),
            uv: TextureRegion { x, .. },
            ..
        } if approx_eq(*x, 0.5)
    )));

    let no_play_snapshot = SelectSnapshot {
        rows: vec![SelectRowSnapshot {
            index: 0,
            ex_score: None,
            play_count: 0,
            total_notes: 2253,
            in_library: true,
            ..SelectRowSnapshot::default()
        }],
        chart_count: 1,
        ..SelectSnapshot::default()
    };
    let no_play_items = document.select_render_items(&sources, &no_play_snapshot);
    let (no_play_state, _) = document.select_draw_state(&no_play_snapshot, None);
    assert_eq!(skin_state_number(154, &no_play_state), None);
    assert!(!no_play_items.iter().any(|item| matches!(
        item,
        SkinRenderItem::Image { texture: SkinTextureId(7) | SkinTextureId(42), .. }
    )));

    let no_play_zero_snapshot = SelectSnapshot {
        rows: vec![SelectRowSnapshot {
            index: 0,
            ex_score: Some(0),
            play_count: 0,
            total_notes: 2253,
            in_library: true,
            ..SelectRowSnapshot::default()
        }],
        chart_count: 1,
        ..SelectSnapshot::default()
    };
    let no_play_zero_items = document.select_render_items(&sources, &no_play_zero_snapshot);
    let (no_play_zero_state, _) = document.select_draw_state(&no_play_zero_snapshot, None);
    assert_eq!(skin_state_number(154, &no_play_zero_state), None);
    assert!(!no_play_zero_items.iter().any(|item| matches!(
        item,
        SkinRenderItem::Image { texture: SkinTextureId(7) | SkinTextureId(42), .. }
    )));
}

#[test]
fn select_diff_number_renders_max_zero_as_positive_row() {
    let document: SkinDocument = serde_json::from_str(
        r#"
            {
                "type": 5,
                "w": 100,
                "h": 100,
                "value": [
                    {
                        "id": "RANK_Diff_Exscore",
                        "src": "num",
                        "x": 0,
                        "y": 0,
                        "w": 120,
                        "h": 40,
                        "divx": 12,
                        "divy": 2,
                        "digit": 4,
                        "ref": 154,
                        "zeropadding": 2
                    }
                ],
                "destination": [
                    {
                        "id": "RANK_Diff_Exscore",
                        "dst": [{"x": 10, "y": 20, "w": 10, "h": 10}]
                    }
                ]
            }
            "#,
    )
    .unwrap();
    let sources = HashMap::from([(
        "num".to_string(),
        SkinDocumentTexture {
            source_id: "num".to_string(),
            texture: SkinTextureId(42),
            source_size: SkinImageSize { width: 120.0, height: 40.0 },
        },
    )]);
    let snapshot = SelectSnapshot {
        rows: vec![SelectRowSnapshot {
            index: 0,
            ex_score: Some(2000),
            total_notes: 1000,
            in_library: true,
            ..SelectRowSnapshot::default()
        }],
        chart_count: 1,
        ..SelectSnapshot::default()
    };

    let items = document.select_render_items(&sources, &snapshot);
    let first_digit_uv = items.iter().find_map(|item| match item {
        SkinRenderItem::Image { texture: SkinTextureId(42), uv, .. } => Some(*uv),
        _ => None,
    });

    let (state, _) = document.select_draw_state(&snapshot, None);
    assert_eq!(skin_state_number(154, &state), Some(0));
    assert_eq!(first_digit_uv.map(|uv| uv.y), Some(0.0));
}

#[test]
fn select_replay_ops_reflect_replay_slots_and_selection() {
    let no_replay = SkinDrawState::default();
    let first_replay = SkinDrawState {
        select_replay_slots: [true, false, false, false],
        select_replay_index: Some(0),
        ..SkinDrawState::default()
    };
    let second_replay = SkinDrawState {
        select_replay_slots: [false, true, false, false],
        select_replay_index: Some(1),
        ..SkinDrawState::default()
    };

    assert!(test_skin_op(196, &[], &no_replay));
    assert!(!test_skin_op(197, &[], &no_replay));
    assert!(!test_skin_op(1205, &[], &no_replay));
    assert!(test_skin_op(197, &[], &first_replay));
    assert!(!test_skin_op(196, &[], &first_replay));
    assert!(test_skin_op(1205, &[], &first_replay));
    assert!(test_skin_op(-1205, &[], &no_replay));
    assert!(test_skin_op(1197, &[], &second_replay));
    assert!(test_skin_op(1206, &[], &second_replay));
    assert!(!test_skin_op(1205, &[], &second_replay));
    assert!(!test_skin_op(198, &[], &first_replay));
}

#[test]
fn select_row_snapshot_carries_achieved_trophy_names() {
    // SelectRowSnapshot is the carrier — SkinDrawState intentionally does
    // not duplicate this field (it must stay Copy).  This test simply
    // pins down that course rows preserve the data and song rows default
    // to empty, so future skin ops have a stable contract to consume.
    use crate::scene::{SelectRowKind, SelectRowSnapshot};
    let course = SelectRowSnapshot {
        kind: SelectRowKind::Course,
        achieved_trophy_names: vec!["gold".to_string(), "silver".to_string()],
        ..SelectRowSnapshot::default()
    };
    let song = SelectRowSnapshot { kind: SelectRowKind::Song, ..SelectRowSnapshot::default() };

    assert_eq!(course.achieved_trophy_names, vec!["gold".to_string(), "silver".to_string()]);
    assert!(song.achieved_trophy_names.is_empty());
}

#[test]
fn select_row_replay_index_is_row_kind_agnostic() {
    // Regression: course rows must surface their replay slot indicators
    // exactly like song rows.  `select_row_replay_index` looks only at
    // `row.replay_slots`, so swapping row.kind must not change the
    // result.  This locks the invariant for future refactors.
    use crate::scene::{SelectRowKind, SelectRowSnapshot};
    let song = SelectRowSnapshot {
        kind: SelectRowKind::Song,
        replay_slots: [false, true, false, true],
        ..SelectRowSnapshot::default()
    };
    let course = SelectRowSnapshot {
        kind: SelectRowKind::Course,
        replay_slots: [false, true, false, true],
        ..SelectRowSnapshot::default()
    };

    assert_eq!(select_row_replay_index(&song, Some(3)), Some(3));
    assert_eq!(select_row_replay_index(&course, Some(3)), Some(3));
    assert_eq!(select_row_replay_index(&song, Some(0)), None);
}

#[test]
fn peaceful_gauge_value_overlay_selects_exactly_one_integer_width() {
    for (state, mode, expected_digits) in [
        (SkinDrawState { gauge: 7.5, gauge_max: 120.0, ..Default::default() }, "percent", 1),
        (SkinDrawState { gauge: 78.75, gauge_max: 120.0, ..Default::default() }, "percent", 2),
        (SkinDrawState { gauge: 120.0, gauge_max: 120.0, ..Default::default() }, "percent", 3),
        (SkinDrawState { gauge: 7.5, gauge_max: 120.0, ..Default::default() }, "amount", 1),
        (SkinDrawState { gauge: 78.75, gauge_max: 120.0, ..Default::default() }, "amount", 2),
        (SkinDrawState { gauge: 120.0, gauge_max: 120.0, ..Default::default() }, "amount", 3),
    ] {
        let visible = (1..=3)
            .filter(|digits| {
                eval_skin_draw_condition(&format!("gauge_value_digits({mode},{digits})"), &state)
            })
            .collect::<Vec<_>>();
        assert_eq!(visible, vec![expected_digits]);
    }
}
