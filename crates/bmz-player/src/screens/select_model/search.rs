use super::*;

/// Loads chart `SelectItem`s for a search query against title / subtitle / artist
/// / subartist / genre. Enrichment (best score, replay slots, difficulty table
/// level) mirrors `load_select_items_in_folder`.
pub fn load_select_items_for_search(
    library_db: &LibraryDatabase,
    score_db: &ScoreDatabase,
    query: &str,
    ln_policy_setting: LnPolicySetting,
) -> Result<Vec<SelectItem>> {
    load_select_items_for_search_for_rule_mode(
        library_db,
        score_db,
        query,
        ln_policy_setting,
        RuleMode::Beatoraja,
    )
}

pub fn load_select_items_for_search_for_rule_mode(
    library_db: &LibraryDatabase,
    score_db: &ScoreDatabase,
    query: &str,
    ln_policy_setting: LnPolicySetting,
    rule_mode: RuleMode,
) -> Result<Vec<SelectItem>> {
    load_select_items_for_search_for_rule_mode_with_table_order(
        library_db,
        score_db,
        query,
        ln_policy_setting,
        rule_mode,
        &[],
    )
}

pub fn load_select_items_for_search_for_rule_mode_with_table_order(
    library_db: &LibraryDatabase,
    score_db: &ScoreDatabase,
    query: &str,
    ln_policy_setting: LnPolicySetting,
    rule_mode: RuleMode,
    table_source_order: &[String],
) -> Result<Vec<SelectItem>> {
    load_select_items_for_search_for_rule_mode_with_filters(
        library_db,
        score_db,
        query,
        ln_policy_setting,
        rule_mode,
        table_source_order,
        None,
        None,
    )
}

pub fn load_select_items_for_search_for_rule_mode_with_filters(
    library_db: &LibraryDatabase,
    score_db: &ScoreDatabase,
    query: &str,
    ln_policy_setting: LnPolicySetting,
    rule_mode: RuleMode,
    table_source_order: &[String],
    active_song_roots: Option<&[String]>,
    active_table_sources: Option<&[String]>,
) -> Result<Vec<SelectItem>> {
    let charts = library_db.search_charts_in_roots(query, active_song_roots)?;
    chart_items_with_enrichment(
        library_db,
        score_db,
        charts,
        ln_policy_setting,
        rule_mode,
        table_source_order,
        active_table_sources,
    )
}
