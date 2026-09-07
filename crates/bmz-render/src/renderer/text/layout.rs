use super::*;

#[cfg(test)]
pub(super) fn text_width_px<F: Font>(
    text: &str,
    font: &FontArc,
    scaled_font: &impl ScaleFont<F>,
) -> f32 {
    text.chars().map(|ch| scaled_font.h_advance(font.glyph_id(ch))).sum()
}

pub(super) fn fallback_char_advance(ch: char, fonts: VectorFontSet<'_>, scale: PxScale) -> f32 {
    let Some((_, font)) = fonts.select(ch) else {
        return 0.0;
    };
    font.as_scaled(scale).h_advance(font.glyph_id(ch))
}

pub(super) fn fallback_text_width_px(text: &str, fonts: VectorFontSet<'_>, scale: PxScale) -> f32 {
    text.chars().map(|ch| fallback_char_advance(ch, fonts, scale)).sum()
}

pub(super) fn bitmap_text_width_px(text: &str, font: &BitmapFont, scale: PxScale) -> f32 {
    text.chars()
        .filter_map(|ch| font.glyphs.get(&ch))
        .map(|glyph| glyph.xadvance as f32 * scale.x)
        .sum()
}

pub(super) fn rasterized_bitmap_glyph_pixels(
    glyph: crate::bitmap_font::BitmapFontGlyph,
    page: &crate::bitmap_font::BitmapFontPage,
    scale: PxScale,
    width: u32,
    height: u32,
) -> Vec<u8> {
    let mut pixels = vec![0; (width * height * 4) as usize];
    let nearest = is_integer_scale(scale.x) && is_integer_scale(scale.y);
    for dst_y in 0..height {
        for dst_x in 0..width {
            let color = if nearest {
                sample_bitmap_glyph_nearest(glyph, page, dst_x, dst_y, scale)
            } else {
                sample_bitmap_glyph_bilinear(glyph, page, dst_x, dst_y, scale)
            };
            let dst_index = ((dst_y * width + dst_x) * 4) as usize;
            if let Some(dst) = pixels.get_mut(dst_index..dst_index + 4)
                && color[3] >= dst[3]
            {
                dst.copy_from_slice(&color);
            }
        }
    }
    pixels
}

pub(super) fn is_integer_scale(scale: f32) -> bool {
    (scale - scale.round()).abs() <= 0.001
}

pub(super) fn sample_bitmap_glyph_nearest(
    glyph: crate::bitmap_font::BitmapFontGlyph,
    page: &crate::bitmap_font::BitmapFontPage,
    dst_x: u32,
    dst_y: u32,
    scale: PxScale,
) -> [u8; 4] {
    let local_x = (dst_x as f32 / scale.x).floor() as i32;
    let local_y = (dst_y as f32 / scale.y).floor() as i32;
    bitmap_glyph_pixel(glyph, page, local_x, local_y)
}

pub(super) fn sample_bitmap_glyph_bilinear(
    glyph: crate::bitmap_font::BitmapFontGlyph,
    page: &crate::bitmap_font::BitmapFontPage,
    dst_x: u32,
    dst_y: u32,
    scale: PxScale,
) -> [u8; 4] {
    let max_x = glyph.width.saturating_sub(1) as f32;
    let max_y = glyph.height.saturating_sub(1) as f32;
    let src_x = ((dst_x as f32 + 0.5) / scale.x - 0.5).clamp(0.0, max_x);
    let src_y = ((dst_y as f32 + 0.5) / scale.y - 0.5).clamp(0.0, max_y);
    let x0 = src_x.floor() as i32;
    let y0 = src_y.floor() as i32;
    let x1 = (x0 + 1).min(max_x as i32);
    let y1 = (y0 + 1).min(max_y as i32);
    let tx = src_x - x0 as f32;
    let ty = src_y - y0 as f32;

    let p00 = bitmap_glyph_pixel(glyph, page, x0, y0);
    let p10 = bitmap_glyph_pixel(glyph, page, x1, y0);
    let p01 = bitmap_glyph_pixel(glyph, page, x0, y1);
    let p11 = bitmap_glyph_pixel(glyph, page, x1, y1);
    blend_bitmap_pixels([
        (p00, (1.0 - tx) * (1.0 - ty)),
        (p10, tx * (1.0 - ty)),
        (p01, (1.0 - tx) * ty),
        (p11, tx * ty),
    ])
}

pub(super) fn bitmap_glyph_pixel(
    glyph: crate::bitmap_font::BitmapFontGlyph,
    page: &crate::bitmap_font::BitmapFontPage,
    local_x: i32,
    local_y: i32,
) -> [u8; 4] {
    if glyph.width == 0 || glyph.height == 0 {
        return [0, 0, 0, 0];
    }
    let local_x = local_x.clamp(0, glyph.width.saturating_sub(1) as i32) as u32;
    let local_y = local_y.clamp(0, glyph.height.saturating_sub(1) as i32) as u32;
    let src_x = glyph.x + local_x;
    let src_y = glyph.y + local_y;
    if src_x >= page.image.width || src_y >= page.image.height {
        return [0, 0, 0, 0];
    }
    let src_index = ((src_y * page.image.width + src_x) * 4) as usize;
    page.image
        .pixels
        .get(src_index..src_index + 4)
        .map(|src| [src[0], src[1], src[2], src[3]])
        .unwrap_or([0, 0, 0, 0])
}

pub(super) fn blend_bitmap_pixels(samples: [([u8; 4], f32); 4]) -> [u8; 4] {
    let mut alpha = 0.0;
    let mut premul = [0.0; 3];
    for (pixel, weight) in samples {
        let a = pixel[3] as f32 / 255.0;
        alpha += a * weight;
        for channel in 0..3 {
            premul[channel] += (pixel[channel] as f32 / 255.0) * a * weight;
        }
    }
    if alpha <= f32::EPSILON {
        return [0, 0, 0, 0];
    }
    [
        ((premul[0] / alpha) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((premul[1] / alpha) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((premul[2] / alpha) * 255.0).round().clamp(0.0, 255.0) as u8,
        (alpha * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

pub(super) fn text_align_offset_px(align: TextAlign, max_width: f32, text_width: f32) -> f32 {
    match align {
        TextAlign::Left => 0.0,
        TextAlign::Center if max_width > 0.0 => (max_width - text_width) / 2.0,
        TextAlign::Center => -text_width / 2.0,
        TextAlign::Right if max_width > 0.0 => max_width - text_width,
        TextAlign::Right => -text_width,
    }
}

#[cfg(test)]
pub(super) fn vector_text_caret_rect(
    origin: &Point,
    text: &str,
    style: &TextStyle,
    font: &FontArc,
    surface: SurfaceSize,
    caret: TextCaret,
) -> Option<RectCommand> {
    if style.wrapping || !surface.is_drawable() {
        return None;
    }
    let px_size = (style.size * surface.height as f32).max(1.0);
    let max_width = style.max_width.max(0.0) * surface.width as f32;
    let mut visible = Cow::Borrowed(text);
    let mut scale = PxScale::from(px_size);
    let original_scale = scale;
    let mut scaled_font = font.as_scaled(scale);
    let mut text_width = text_width_px(&visible, font, &scaled_font);
    if max_width > 0.0 && text_width > max_width {
        match style.overflow {
            TextOverflow::Overflow => {}
            TextOverflow::Shrink => {
                scale.x = (scale.x * max_width / text_width).max(0.01);
                scaled_font = font.as_scaled(scale);
                text_width = text_width_px(&visible, font, &scaled_font);
            }
            TextOverflow::ShrinkUniform => {
                scale = PxScale::from((scale.x * max_width / text_width).max(1.0));
                scaled_font = font.as_scaled(scale);
                text_width = text_width_px(&visible, font, &scaled_font);
            }
            TextOverflow::Truncate => {
                visible =
                    Cow::Owned(truncate_text_to_width(&visible, font, &scaled_font, max_width));
                text_width = text_width_px(&visible, font, &scaled_font);
            }
        }
    }
    let align_offset = text_align_offset_px(style.align, max_width, text_width);
    let cursor = clamp_text_byte_index(&visible, caret.byte_index);
    let prefix_width = text_width_px(&visible[..cursor], font, &scaled_font);
    let shrink_offset_y =
        if matches!(style.overflow, TextOverflow::ShrinkUniform) && scale.y < original_scale.y {
            (original_scale.y - scale.y) / 2.0
        } else {
            0.0
        };
    let x = (origin.x * surface.width as f32 + align_offset + prefix_width) / surface.width as f32;
    let y = (origin.y * surface.height as f32 + shrink_offset_y) / surface.height as f32;
    Some(RectCommand {
        rect: Rect {
            x,
            y,
            width: (2.0 / surface.width as f32).max(0.001),
            height: (scale.y / surface.height as f32).max(0.001),
        },
        color: caret.color,
    })
}

pub(super) fn vector_text_caret_rect_with_fallback(
    origin: &Point,
    text: &str,
    style: &TextStyle,
    fonts: VectorFontSet<'_>,
    surface: SurfaceSize,
    caret: TextCaret,
) -> Option<RectCommand> {
    if style.wrapping || !surface.is_drawable() || fonts.primary().is_none() {
        return None;
    }
    let px_size = (style.size * surface.height as f32).max(1.0);
    let max_width = style.max_width.max(0.0) * surface.width as f32;
    let mut visible = Cow::Borrowed(text);
    let mut scale = PxScale::from(px_size);
    let original_scale = scale;
    let mut text_width = fallback_text_width_px(&visible, fonts, scale);
    if max_width > 0.0 && text_width > max_width {
        match style.overflow {
            TextOverflow::Overflow => {}
            TextOverflow::Shrink => {
                scale.x = (scale.x * max_width / text_width).max(0.01);
                text_width = fallback_text_width_px(&visible, fonts, scale);
            }
            TextOverflow::ShrinkUniform => {
                scale = PxScale::from((scale.x * max_width / text_width).max(1.0));
                text_width = fallback_text_width_px(&visible, fonts, scale);
            }
            TextOverflow::Truncate => {
                visible =
                    Cow::Owned(truncate_fallback_text_to_width(&visible, fonts, scale, max_width));
                text_width = fallback_text_width_px(&visible, fonts, scale);
            }
        }
    }
    let align_offset = text_align_offset_px(style.align, max_width, text_width);
    let cursor = clamp_text_byte_index(&visible, caret.byte_index);
    let prefix_width = fallback_text_width_px(&visible[..cursor], fonts, scale);
    let shrink_offset_y =
        if matches!(style.overflow, TextOverflow::ShrinkUniform) && scale.y < original_scale.y {
            (original_scale.y - scale.y) / 2.0
        } else {
            0.0
        };
    let x = (origin.x * surface.width as f32 + align_offset + prefix_width) / surface.width as f32;
    let y = (origin.y * surface.height as f32 + shrink_offset_y) / surface.height as f32;
    Some(RectCommand {
        rect: Rect {
            x,
            y,
            width: (2.0 / surface.width as f32).max(0.001),
            height: (scale.y / surface.height as f32).max(0.001),
        },
        color: caret.color,
    })
}

pub(super) fn bitmap_text_caret_rect(
    origin: &Point,
    text: &str,
    style: &TextStyle,
    font: &BitmapFont,
    surface: SurfaceSize,
    caret: TextCaret,
) -> Option<RectCommand> {
    if style.wrapping || !surface.is_drawable() {
        return None;
    }
    let design_size = if font.size > 0 { font.size } else { font.line_height.max(1) };
    let bitmap_size = style.bitmap_size.unwrap_or(style.size);
    let mut scale =
        PxScale::from((bitmap_size * surface.height as f32 / design_size as f32).max(0.01));
    let original_scale = scale;
    let max_width = style.max_width.max(0.0) * surface.width as f32;
    let mut text_width = bitmap_text_width_px(text, font, scale);
    let visible = if max_width > 0.0 && text_width > max_width {
        match style.overflow {
            TextOverflow::Overflow => Cow::Borrowed(text),
            TextOverflow::Shrink => {
                scale.x = (scale.x * max_width / text_width).max(0.01);
                Cow::Borrowed(text)
            }
            TextOverflow::ShrinkUniform => {
                scale = PxScale::from((scale.x * max_width / text_width).max(0.01));
                Cow::Borrowed(text)
            }
            TextOverflow::Truncate => {
                Cow::Owned(truncate_bitmap_text_to_width(text, font, max_width, scale))
            }
        }
    } else {
        Cow::Borrowed(text)
    };
    text_width = bitmap_text_width_px(&visible, font, scale);
    let align_offset = text_align_offset_px(style.align, max_width, text_width);
    let cursor = clamp_text_byte_index(&visible, caret.byte_index);
    let prefix_width = bitmap_text_width_px(&visible[..cursor], font, scale);
    let shrink_offset_y =
        if matches!(style.overflow, TextOverflow::ShrinkUniform) && scale.y < original_scale.y {
            (design_size as f32 * (original_scale.y - scale.y)) / 2.0
        } else {
            0.0
        };
    let caret_height = design_size as f32 * scale.y;
    let x = (origin.x * surface.width as f32 + align_offset + prefix_width) / surface.width as f32;
    let y = (origin.y * surface.height as f32 + shrink_offset_y) / surface.height as f32;
    Some(RectCommand {
        rect: Rect {
            x,
            y,
            width: (2.0 / surface.width as f32).max(0.001),
            height: (caret_height / surface.height as f32).max(0.001),
        },
        color: caret.color,
    })
}

pub(super) fn clamp_text_byte_index(text: &str, byte_index: usize) -> usize {
    let mut byte_index = byte_index.min(text.len());
    while byte_index > 0 && !text.is_char_boundary(byte_index) {
        byte_index -= 1;
    }
    byte_index
}

pub(super) fn truncate_bitmap_text_to_width(
    text: &str,
    font: &BitmapFont,
    max_width: f32,
    scale: PxScale,
) -> String {
    let mut width = 0.0;
    let mut result = String::new();
    for ch in text.chars() {
        let Some(glyph) = font.glyphs.get(&ch) else {
            continue;
        };
        let advance = glyph.xadvance as f32 * scale.x;
        if width + advance > max_width {
            break;
        }
        width += advance;
        result.push(ch);
    }
    result
}

#[cfg(test)]
pub(super) fn wrap_text_to_width<F: Font>(
    text: &str,
    font: &FontArc,
    scaled_font: &impl ScaleFont<F>,
    max_width: f32,
) -> Vec<String> {
    let mut lines = Vec::new();
    for source_line in text.split('\n') {
        let mut line = String::new();
        let mut width = 0.0;
        for ch in source_line.chars() {
            let advance = scaled_font.h_advance(font.glyph_id(ch));
            if !line.is_empty() && width + advance > max_width {
                lines.push(std::mem::take(&mut line));
                width = 0.0;
            }
            line.push(ch);
            width += advance;
        }
        lines.push(line);
    }
    lines
}

pub(super) fn wrap_fallback_text_to_width(
    text: &str,
    fonts: VectorFontSet<'_>,
    scale: PxScale,
    max_width: f32,
) -> Vec<String> {
    let mut lines = Vec::new();
    for source_line in text.split('\n') {
        let mut line = String::new();
        let mut width = 0.0;
        for ch in source_line.chars() {
            let advance = fallback_char_advance(ch, fonts, scale);
            if !line.is_empty() && width + advance > max_width {
                lines.push(std::mem::take(&mut line));
                width = 0.0;
            }
            line.push(ch);
            width += advance;
        }
        lines.push(line);
    }
    lines
}

#[cfg(test)]
pub(super) fn truncate_text_to_width<F: Font>(
    text: &str,
    font: &FontArc,
    scaled_font: &impl ScaleFont<F>,
    max_width: f32,
) -> String {
    let mut width = 0.0;
    let mut result = String::new();
    for ch in text.chars() {
        let advance = scaled_font.h_advance(font.glyph_id(ch));
        if width + advance > max_width {
            break;
        }
        width += advance;
        result.push(ch);
    }
    result
}

pub(super) fn truncate_fallback_text_to_width(
    text: &str,
    fonts: VectorFontSet<'_>,
    scale: PxScale,
    max_width: f32,
) -> String {
    let mut width = 0.0;
    let mut result = String::new();
    for ch in text.chars() {
        let advance = fallback_char_advance(ch, fonts, scale);
        if width + advance > max_width {
            break;
        }
        width += advance;
        result.push(ch);
    }
    result
}

pub(super) fn encode_text_quads(
    quads: &[TextQuad],
    atlas_width: u32,
    atlas_height: u32,
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(quads.len() * TEXT_INSTANCE_BYTES);
    for quad in quads {
        for value in [
            quad.x,
            quad.y,
            quad.width,
            quad.height,
            quad.atlas_origin.0 as f32 / atlas_width as f32,
            quad.atlas_origin.1 as f32 / atlas_height as f32,
            quad.glyph_width as f32 / atlas_width as f32,
            quad.glyph_height as f32 / atlas_height as f32,
            quad.color.r,
            quad.color.g,
            quad.color.b,
            quad.color.a,
        ] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
    }
    bytes
}
