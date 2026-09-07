use super::*;

pub(super) fn pm_chara_render_items(
    definition: &SkinPmCharaDef,
    destination: &SkinDestinationDef,
    outer_frame: ResolvedSkinFrame,
    destination_elapsed_ms: i32,
    state: &SkinDrawState,
    sources: &HashMap<String, SkinDocumentTexture>,
    canvas_width: u32,
    canvas_height: u32,
) -> Vec<SkinRenderItem> {
    let Some(runtime) = &definition.runtime else {
        return Vec::new();
    };
    let (motion, motion_elapsed_ms) =
        pm_chara_motion(definition, runtime, state).unwrap_or((0, destination_elapsed_ms));
    let mut selected_motion = motion;
    if !runtime.motions.iter().any(|layer| layer.motion == selected_motion) {
        selected_motion = runtime
            .motions
            .iter()
            .find(|layer| layer.motion == 1)
            .or_else(|| runtime.motions.first())
            .map_or(0, |layer| layer.motion);
    }

    let outer_rect = normalize_skin_frame_rect(outer_frame, canvas_width, canvas_height);
    let pm_width = runtime.canvas_width.max(1) as f32;
    let pm_height = runtime.canvas_height.max(1) as f32;
    runtime
        .motions
        .iter()
        .filter(|layer| layer.motion == selected_motion)
        .filter_map(|layer| {
            let frame = pm_chara_layer_frame(layer, motion_elapsed_ms)?;
            if frame.source_w == 0 || frame.source_h == 0 {
                return None;
            }
            let source = resolve_document_source(sources, &layer.source_id)?;
            let (source_x, source_y, source_w, source_h) = resolve_skin_image_pixel_rect(
                (frame.source_x, frame.source_y, frame.source_w, frame.source_h),
                source.source_size.width.max(1.0),
                source.source_size.height.max(1.0),
            );
            let uv = TextureRegion {
                x: source_x as f32 / source.source_size.width.max(1.0),
                y: source_y as f32 / source.source_size.height.max(1.0),
                width: source_w as f32 / source.source_size.width.max(1.0),
                height: source_h as f32 / source.source_size.height.max(1.0),
            };
            let rect = Rect {
                x: outer_rect.x + outer_rect.width * frame.destination_x as f32 / pm_width,
                y: outer_rect.y + outer_rect.height * frame.destination_y as f32 / pm_height,
                width: outer_rect.width * frame.destination_w as f32 / pm_width,
                height: outer_rect.height * frame.destination_h as f32 / pm_height,
            };
            let mut item_frame = outer_frame;
            item_frame.a = (item_frame.a * frame.alpha.clamp(0, 255) / 255).clamp(0, 255);
            item_frame.angle = item_frame.angle.saturating_add(frame.angle);
            Some(skin_image_item_for_frame(
                source.texture,
                rect,
                uv,
                item_frame,
                destination.center,
                skin_blend_mode(destination.blend),
                Some(source.source_size),
                destination.filter != 0,
            ))
        })
        .collect()
}

fn pm_chara_motion(
    definition: &SkinPmCharaDef,
    runtime: &SkinPmCharaRuntimeDef,
    state: &SkinDrawState,
) -> Option<(i32, i32)> {
    if definition.chara_type != 0 {
        let motion = match definition.chara_type {
            1..=5 => 0,
            6 => 1,
            7 => 6,
            8 => 7,
            9 => 8,
            10 => 10,
            11 => 17,
            12 => 15,
            13 => 16,
            14 => 3,
            15 => 14,
            _ => return None,
        };
        return Some((motion, state.elapsed_ms));
    }

    if let Some(elapsed) = state.music_end_ms {
        let motion = if definition.side == 2 {
            if state.gauge >= state.gauge_border { 16 } else { 15 }
        } else if state.gauge >= state.gauge_max {
            17
        } else if state.gauge >= state.gauge_border {
            15
        } else {
            16
        };
        if runtime.motions.iter().any(|layer| layer.motion == motion) {
            return Some((motion, elapsed));
        }
    }

    if let (Some(elapsed), Some(judge)) = (state.judge_ms[0], state.judge_index[0]) {
        let motion = if definition.side == 2 {
            match judge {
                0..=2 => 10,
                _ => 7,
            }
        } else {
            match judge {
                0 | 1 if state.gauge >= state.gauge_max => 6,
                0 | 1 => 7,
                2 => 8,
                _ => 10,
            }
        };
        let duration = runtime
            .motions
            .iter()
            .filter(|layer| layer.motion == motion)
            .map(pm_chara_layer_cycle_ms)
            .max()
            .unwrap_or(0);
        if elapsed < duration {
            return Some((motion, elapsed));
        }
    }

    Some((1, state.play_timer_ms.unwrap_or(state.elapsed_ms)))
}

fn pm_chara_layer_frame(
    layer: &SkinPmCharaMotionLayerDef,
    elapsed_ms: i32,
) -> Option<SkinPmCharaFrameDef> {
    if layer.frames.is_empty() {
        return None;
    }
    let tick = elapsed_ms.max(0) as usize / layer.frame_ms.max(1) as usize;
    let loop_start = layer.loop_start.min(layer.frames.len().saturating_sub(1));
    let index = if tick < loop_start {
        tick
    } else {
        let loop_len = layer.frames.len() - loop_start;
        loop_start + (tick - loop_start) % loop_len.max(1)
    };
    layer.frames.get(index).copied()
}

fn pm_chara_layer_cycle_ms(layer: &SkinPmCharaMotionLayerDef) -> i32 {
    layer.frame_ms.max(1).saturating_mul(layer.frames.len().min(i32::MAX as usize) as i32)
}
