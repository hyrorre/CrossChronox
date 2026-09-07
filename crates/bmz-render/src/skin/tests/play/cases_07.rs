use super::*;

fn pm_chara_document() -> SkinDocument {
    let mut document: SkinDocument = serde_json::from_str(
        r#"
            {
                "type": 0,
                "w": 100,
                "h": 100,
                "pmchara": [
                    { "id": "pm", "src": "pm-source", "color": 1, "type": 0, "side": 1 }
                ],
                "destination": [
                    { "id": "pm", "dst": [{ "x": 10, "y": 20, "w": 40, "h": 60 }] }
                ]
            }
        "#,
    )
    .unwrap();
    document.pmchara[0].runtime = Some(SkinPmCharaRuntimeDef {
        canvas_width: 20,
        canvas_height: 30,
        motions: vec![
            SkinPmCharaMotionLayerDef {
                motion: 1,
                source_id: "pm-bmp".to_string(),
                frame_ms: 100,
                loop_start: 0,
                frames: vec![
                    SkinPmCharaFrameDef {
                        source_x: 0,
                        source_y: 0,
                        source_w: 20,
                        source_h: 30,
                        destination_x: 0,
                        destination_y: 0,
                        destination_w: 20,
                        destination_h: 30,
                        alpha: 255,
                        angle: 0,
                    },
                    SkinPmCharaFrameDef {
                        source_x: 20,
                        source_y: 0,
                        source_w: 20,
                        source_h: 30,
                        destination_x: 5,
                        destination_y: 0,
                        destination_w: 10,
                        destination_h: 30,
                        alpha: 128,
                        angle: 0,
                    },
                ],
            },
            SkinPmCharaMotionLayerDef {
                motion: 7,
                source_id: "pm-bmp".to_string(),
                frame_ms: 200,
                loop_start: 0,
                frames: vec![SkinPmCharaFrameDef {
                    source_x: 20,
                    source_y: 0,
                    source_w: 20,
                    source_h: 30,
                    destination_x: 0,
                    destination_y: 0,
                    destination_w: 20,
                    destination_h: 30,
                    alpha: 255,
                    angle: 0,
                }],
            },
        ],
    });
    document
}

fn pm_chara_sources() -> HashMap<String, SkinDocumentTexture> {
    HashMap::from([(
        "pm-bmp".to_string(),
        SkinDocumentTexture {
            source_id: "pm-bmp".to_string(),
            texture: SkinTextureId(77),
            source_size: SkinImageSize { width: 40.0, height: 30.0 },
        },
    )])
}

fn single_frame_pm_chara_motion(motion: i32, source_id: &str) -> SkinPmCharaMotionLayerDef {
    SkinPmCharaMotionLayerDef {
        motion,
        source_id: source_id.to_string(),
        frame_ms: 200,
        loop_start: 0,
        frames: vec![SkinPmCharaFrameDef {
            source_x: 0,
            source_y: 0,
            source_w: 20,
            source_h: 30,
            destination_x: 0,
            destination_y: 0,
            destination_w: 20,
            destination_h: 30,
            alpha: 255,
            angle: 0,
        }],
    }
}

fn insert_pm_chara_source(
    sources: &mut HashMap<String, SkinDocumentTexture>,
    source_id: &str,
    texture: SkinTextureId,
) {
    sources.insert(
        source_id.to_string(),
        SkinDocumentTexture {
            source_id: source_id.to_string(),
            texture,
            source_size: SkinImageSize { width: 20.0, height: 30.0 },
        },
    );
}

#[test]
fn pm_chara_renders_neutral_animation_frames_and_inner_geometry() {
    let document = pm_chara_document();
    let sources = pm_chara_sources();

    let first = document.static_image_render_items(
        &sources,
        &SkinDrawState { elapsed_ms: 0, play_timer_ms: Some(0), ..Default::default() },
    );
    let second = document.static_image_render_items(
        &sources,
        &SkinDrawState { elapsed_ms: 100, play_timer_ms: Some(100), ..Default::default() },
    );

    assert!(matches!(
        first.as_slice(),
        [SkinRenderItem::Image {
            texture: SkinTextureId(77),
            rect: Rect { x, y, width, height },
            uv: TextureRegion { x: uv_x, width: uv_width, .. },
            ..
        }] if approx_eq(*x, 0.1)
            && approx_eq(*y, 0.2)
            && approx_eq(*width, 0.4)
            && approx_eq(*height, 0.6)
            && approx_eq(*uv_x, 0.0)
            && approx_eq(*uv_width, 0.5)
    ));
    assert!(matches!(
        second.as_slice(),
        [SkinRenderItem::Image {
            rect: Rect { x, width, .. },
            uv: TextureRegion { x: uv_x, .. },
            tint: Color { a, .. },
            ..
        }] if approx_eq(*x, 0.2)
            && approx_eq(*width, 0.2)
            && approx_eq(*uv_x, 0.5)
            && approx_eq(*a, 128.0 / 255.0)
    ));
}

#[test]
fn pm_chara_uses_recent_great_reaction_motion() {
    let document = pm_chara_document();
    let sources = pm_chara_sources();
    let mut state = SkinDrawState { elapsed_ms: 1_000, gauge: 50.0, ..Default::default() };
    state.judge_ms[0] = Some(0);
    state.judge_index[0] = Some(1);

    let items = document.static_image_render_items(&sources, &state);

    assert!(matches!(
        items.as_slice(),
        [SkinRenderItem::Image { uv: TextureRegion { x, .. }, .. }] if approx_eq(*x, 0.5)
    ));
}

#[test]
fn pm_chara_side_two_uses_opponent_judgement_reactions() {
    let mut document = pm_chara_document();
    document.pmchara[0].side = 2;
    document.pmchara[0]
        .runtime
        .as_mut()
        .unwrap()
        .motions
        .push(single_frame_pm_chara_motion(10, "pm-bad"));
    let mut sources = pm_chara_sources();
    insert_pm_chara_source(&mut sources, "pm-bad", SkinTextureId(78));
    let mut state = SkinDrawState { elapsed_ms: 1_000, gauge: 50.0, ..Default::default() };
    state.judge_ms[0] = Some(0);
    state.judge_index[0] = Some(0);

    let player_success = document.static_image_render_items(&sources, &state);
    state.judge_index[0] = Some(3);
    let player_failure = document.static_image_render_items(&sources, &state);

    assert!(matches!(
        player_success.as_slice(),
        [SkinRenderItem::Image { texture: SkinTextureId(78), .. }]
    ));
    assert!(matches!(
        player_failure.as_slice(),
        [SkinRenderItem::Image {
            texture: SkinTextureId(77),
            uv: TextureRegion { x, .. },
            ..
        }] if approx_eq(*x, 0.5)
    ));
}

#[test]
fn pm_chara_side_two_inverts_music_end_result() {
    let mut document = pm_chara_document();
    document.pmchara[0].side = 2;
    document.pmchara[0].runtime.as_mut().unwrap().motions.extend([
        single_frame_pm_chara_motion(15, "pm-win"),
        single_frame_pm_chara_motion(16, "pm-lose"),
        single_frame_pm_chara_motion(17, "pm-fever-win"),
    ]);
    let mut sources = pm_chara_sources();
    insert_pm_chara_source(&mut sources, "pm-win", SkinTextureId(79));
    insert_pm_chara_source(&mut sources, "pm-lose", SkinTextureId(80));
    insert_pm_chara_source(&mut sources, "pm-fever-win", SkinTextureId(81));

    let player_clear = document.static_image_render_items(
        &sources,
        &SkinDrawState {
            music_end_ms: Some(0),
            gauge: 100.0,
            gauge_border: 80.0,
            gauge_max: 100.0,
            ..Default::default()
        },
    );
    let player_failed = document.static_image_render_items(
        &sources,
        &SkinDrawState {
            music_end_ms: Some(0),
            gauge: 50.0,
            gauge_border: 80.0,
            gauge_max: 100.0,
            ..Default::default()
        },
    );

    assert!(matches!(
        player_clear.as_slice(),
        [SkinRenderItem::Image { texture: SkinTextureId(80), .. }]
    ));
    assert!(matches!(
        player_failed.as_slice(),
        [SkinRenderItem::Image { texture: SkinTextureId(79), .. }]
    ));
}
