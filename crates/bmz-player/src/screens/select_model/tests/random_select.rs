use super::*;
use crate::config::profile_config::SelectStateConfig;

fn all_enabled() -> SelectStateConfig {
    SelectStateConfig {
        random_select: true,
        random_select_no_play: true,
        random_select_failed: true,
        random_select_not_easy: true,
        random_select_not_clear: true,
        random_select_not_hard: true,
        random_select_not_ex_hard: true,
        random_select_not_full_combo: true,
        ..Default::default()
    }
}

fn bars(items: &[SelectItem], config: &SelectStateConfig) -> Vec<(String, Vec<i64>)> {
    random_select_items_from_items(items, config)
        .into_iter()
        .map(|item| {
            let SelectItem::Executable(row) = item else {
                panic!("expected executable");
            };
            assert_eq!(row.kind, SelectExecutableKind::RandomSelect);
            (row.title, row.chart_ids)
        })
        .collect()
}

#[test]
fn random_select_uses_persisted_scores_and_all_clear_boundaries() {
    let (mut library, mut scores) = open_in_memory_dbs();
    let unplayed = chart("Unplayed");
    library.upsert_chart_import(&record_for_chart("/songs/unplayed.bms", &unplayed)).unwrap();
    let clears = [
        ClearType::NoPlay,
        ClearType::Failed,
        ClearType::AssistEasy,
        ClearType::LightAssistEasy,
        ClearType::Easy,
        ClearType::Normal,
        ClearType::Hard,
        ClearType::ExHard,
        ClearType::FullCombo,
        ClearType::Perfect,
        ClearType::Max,
    ];
    for clear in clears {
        let chart = chart(clear.as_str());
        library
            .upsert_chart_import(&record_for_chart(
                &format!("/songs/{}.bms", clear.as_str()),
                &chart,
            ))
            .unwrap();
        let mut record = score_for_chart(chart.identity.file_sha256);
        record.clear_type = clear;
        scores.insert_score(&record).unwrap();
    }
    let load = |scores: &ScoreDatabase| {
        load_select_items_in_folder(&library, scores, "/songs", LnPolicySetting::AutoLn).unwrap()
    };
    let items = load(&scores);
    let result = bars(&items, &all_enabled());
    assert_eq!(
        result.iter().map(|(_, ids)| ids.len()).collect::<Vec<_>>(),
        vec![12, 1, 1, 5, 6, 7, 8, 9]
    );
    for (index, threshold) in [
        (3, ClearType::Easy),
        (4, ClearType::Normal),
        (5, ClearType::Hard),
        (6, ClearType::ExHard),
        (7, ClearType::FullCombo),
    ] {
        for item in &items {
            let SelectItem::Chart(row) = item else {
                continue;
            };
            let expected = row.best_score.as_ref().is_none_or(|score| {
                ClearType::rank_from_label(&score.clear_type) < threshold as u8
            });
            assert_eq!(result[index].1.contains(&row.chart.as_ref().unwrap().chart_id), expected);
        }
    }
    // A successful play removes a chart from NO PLAY on the next list reload.
    scores.insert_score(&score_for_chart(unplayed.identity.file_sha256)).unwrap();
    assert!(
        !bars(&load(&scores), &all_enabled())
            .iter()
            .any(|(name, _)| name == "NO PLAY RANDOM SELECT")
    );
}

#[test]
fn random_select_respects_individual_flags_minimum_counts_and_missing_charts() {
    let (mut library, mut scores) = open_in_memory_dbs();
    let chart = chart("Single");
    library.upsert_chart_import(&record_for_chart("/songs/single.bms", &chart)).unwrap();
    scores.insert_score(&score_for_chart(chart.identity.file_sha256)).unwrap();
    let mut items =
        load_select_items_in_folder(&library, &scores, "/songs", LnPolicySetting::AutoLn).unwrap();
    let SelectItem::Chart(row) = &mut items[0] else {
        panic!("expected chart");
    };
    // Imported score data can have a lamp even when play_count is zero.
    row.best_score.as_mut().unwrap().play_count = 0;
    let mut missing = row.clone();
    missing.chart = None;
    items.push(SelectItem::Chart(missing));
    items.push(random_mix_item());
    items.extend(root_folder_items(&["/other".to_string()]));
    assert!(bars(&items, &SelectStateConfig::default()).is_empty());
    let config = SelectStateConfig {
        random_select: true,
        random_select_no_play: true,
        ..Default::default()
    };
    let result = bars(&items, &config);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].0, "NO PLAY RANDOM SELECT");
    assert_eq!(result[0].1.len(), 1);
    assert!(bars(&[], &all_enabled()).is_empty());
}

#[test]
fn random_select_legacy_config_keeps_new_types_disabled() {
    let config: SelectStateConfig = toml::from_str("random_select = true").unwrap();
    assert_eq!(
        config.random_select_flags(),
        [true, false, false, false, false, false, false, false]
    );
}
