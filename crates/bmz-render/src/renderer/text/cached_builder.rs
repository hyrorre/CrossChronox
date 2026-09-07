use super::*;

pub(super) struct CachedTextFrameBuilder<'a> {
    atlas: &'a mut TextAtlasCache,
    pub(super) quads: Vec<TextQuad>,
}

impl<'a> CachedTextFrameBuilder<'a> {
    pub(super) fn new(atlas: &'a mut TextAtlasCache) -> Self {
        Self { atlas, quads: Vec::new() }
    }

    pub(super) fn push_bitmap_text(
        &mut self,
        origin: &Point,
        text: &str,
        style: TextStyle,
        font_id: &str,
        font: &BitmapFont,
        surface: SurfaceSize,
    ) {
        if let Some(shadow) = style.shadow.filter(|shadow| shadow.color.a > 0.0) {
            let mut shadow_style = style.clone();
            shadow_style.color = shadow.color;
            shadow_style.outline = None;
            shadow_style.shadow = None;
            let shadow_origin =
                Point { x: origin.x + shadow.offset.x, y: origin.y + shadow.offset.y };
            self.push_bitmap_text(&shadow_origin, text, shadow_style, font_id, font, surface);
        }
        if let Some(outline) = style.outline.filter(|outline| outline.color.a > 0.0) {
            let mut outline_style = style.clone();
            outline_style.color = outline.color;
            outline_style.outline = None;
            outline_style.shadow = None;
            let outline_y = outline.width;
            let outline_x = outline.width * surface.height as f32 / surface.width as f32;
            for (offset_x, offset_y) in [
                (-outline_x, -outline_y),
                (0.0, -outline_y),
                (outline_x, -outline_y),
                (-outline_x, 0.0),
                (outline_x, 0.0),
                (-outline_x, outline_y),
                (0.0, outline_y),
                (outline_x, outline_y),
            ] {
                let outline_origin = Point { x: origin.x + offset_x, y: origin.y + offset_y };
                self.push_bitmap_text(
                    &outline_origin,
                    text,
                    outline_style.clone(),
                    font_id,
                    font,
                    surface,
                );
            }
        }

        let layout_key =
            text_layout_key(TextGlyphKind::Bitmap, font_id, origin, text, &style, surface);
        if let Some(cached) = layout_key.as_ref().and_then(|key| self.atlas.cached_layout(key)) {
            self.quads.extend_from_slice(cached);
            return;
        }
        let quads_before = self.quads.len();

        let design_size = if font.size > 0 { font.size } else { font.line_height.max(1) };
        let bitmap_size = style.bitmap_size.unwrap_or(style.size);
        let mut scale =
            PxScale::from((bitmap_size * surface.height as f32 / design_size as f32).max(0.01));
        let original_scale = scale;
        let mut text_width = bitmap_text_width_px(text, font, scale);
        let max_width = style.max_width.max(0.0) * surface.width as f32;
        let text = if max_width > 0.0 && text_width > max_width {
            match style.overflow {
                TextOverflow::Overflow => std::borrow::Cow::Borrowed(text),
                TextOverflow::Shrink => {
                    scale.x = (scale.x * max_width / text_width).max(0.01);
                    std::borrow::Cow::Borrowed(text)
                }
                TextOverflow::ShrinkUniform => {
                    scale = PxScale::from((scale.x * max_width / text_width).max(0.01));
                    std::borrow::Cow::Borrowed(text)
                }
                TextOverflow::Truncate => std::borrow::Cow::Owned(truncate_bitmap_text_to_width(
                    text, font, max_width, scale,
                )),
            }
        } else {
            std::borrow::Cow::Borrowed(text)
        };
        text_width = bitmap_text_width_px(&text, font, scale);
        let align_offset = text_align_offset_px(style.align, max_width, text_width);
        let mut cursor_x = origin.x * surface.width as f32 + align_offset;
        let shrink_offset_y = if matches!(style.overflow, TextOverflow::ShrinkUniform)
            && scale.y < original_scale.y
        {
            (design_size as f32 * (original_scale.y - scale.y)) / 2.0
        } else {
            0.0
        };
        let text_top_y = origin.y * surface.height as f32 + shrink_offset_y;

        for ch in text.chars() {
            let Some(glyph) = font.glyphs.get(&ch) else {
                continue;
            };
            if glyph.width > 0
                && glyph.height > 0
                && let Some(page) = font.pages.get(&glyph.page)
            {
                let cached = self.atlas.cached_bitmap_glyph(font_id, ch, *glyph, page, font, scale);
                self.quads.push(TextQuad {
                    x: (cursor_x + cached.offset_x) / surface.width as f32,
                    y: (text_top_y + cached.offset_y) / surface.height as f32,
                    width: cached.display_width / surface.width as f32,
                    height: cached.display_height / surface.height as f32,
                    atlas_origin: cached.atlas_origin,
                    glyph_width: cached.width,
                    glyph_height: cached.height,
                    color: style.color,
                });
            }
            cursor_x += glyph.xadvance as f32 * scale.x;
        }

        if let Some(key) = layout_key {
            self.atlas.insert_layout(key, &self.quads[quads_before..]);
        }
    }

    pub(super) fn push_text(
        &mut self,
        origin: &Point,
        text: &str,
        style: TextStyle,
        fonts: VectorFontSet<'_>,
        surface: SurfaceSize,
    ) {
        if let Some(shadow) = style.shadow.filter(|shadow| shadow.color.a > 0.0) {
            let mut shadow_style = style.clone();
            shadow_style.color = shadow.color;
            shadow_style.outline = None;
            shadow_style.shadow = None;
            let shadow_origin =
                Point { x: origin.x + shadow.offset.x, y: origin.y + shadow.offset.y };
            self.push_text(&shadow_origin, text, shadow_style, fonts, surface);
        }
        if let Some(outline) = style.outline.filter(|outline| outline.color.a > 0.0) {
            let mut outline_style = style.clone();
            outline_style.color = outline.color;
            outline_style.outline = None;
            outline_style.shadow = None;
            let outline_y = outline.width;
            let outline_x = outline.width * surface.height as f32 / surface.width as f32;
            for (offset_x, offset_y) in [
                (-outline_x, -outline_y),
                (0.0, -outline_y),
                (outline_x, -outline_y),
                (-outline_x, 0.0),
                (outline_x, 0.0),
                (-outline_x, outline_y),
                (0.0, outline_y),
                (outline_x, outline_y),
            ] {
                let outline_origin = Point { x: origin.x + offset_x, y: origin.y + offset_y };
                self.push_text(&outline_origin, text, outline_style.clone(), fonts, surface);
            }
        }

        let layout_font_id = fonts.layout_cache_id(text);
        let layout_key =
            text_layout_key(TextGlyphKind::Vector, &layout_font_id, origin, text, &style, surface);
        if let Some(cached) = layout_key.as_ref().and_then(|key| self.atlas.cached_layout(key)) {
            self.quads.extend_from_slice(cached);
            return;
        }
        let quads_before = self.quads.len();

        let px_size = (style.size * surface.height as f32).max(1.0);
        let max_width = style.max_width.max(0.0) * surface.width as f32;
        let mut text = std::borrow::Cow::Borrowed(text);
        let mut scale = PxScale::from(px_size);
        let original_scale = scale;
        let Some((_, primary_font)) = fonts.primary() else {
            return;
        };
        let mut scaled_primary = primary_font.as_scaled(scale);
        let mut text_width = fallback_text_width_px(&text, fonts, scale);
        if style.wrapping && max_width > 0.0 {
            let lines = wrap_fallback_text_to_width(&text, fonts, scale, max_width);
            let line_height = (scaled_primary.ascent() - scaled_primary.descent()
                + scaled_primary.line_gap())
            .max(px_size);
            let origin_x = origin.x * surface.width as f32;
            let first_baseline_y = origin.y * surface.height as f32 + scaled_primary.ascent();
            for (index, line) in lines.iter().enumerate() {
                let line_width = fallback_text_width_px(line, fonts, scale);
                let baseline_y = first_baseline_y + line_height * index as f32;
                self.push_text_line(
                    origin_x,
                    baseline_y,
                    line,
                    line_width,
                    scale,
                    style.clone(),
                    fonts,
                    surface,
                );
            }
            return;
        }
        if max_width > 0.0 && text_width > max_width {
            match style.overflow {
                TextOverflow::Overflow => {}
                TextOverflow::Shrink => {
                    scale.x = (scale.x * max_width / text_width).max(0.01);
                    scaled_primary = primary_font.as_scaled(scale);
                    text_width = fallback_text_width_px(&text, fonts, scale);
                }
                TextOverflow::ShrinkUniform => {
                    scale = PxScale::from((scale.x * max_width / text_width).max(1.0));
                    scaled_primary = primary_font.as_scaled(scale);
                    text_width = fallback_text_width_px(&text, fonts, scale);
                }
                TextOverflow::Truncate => {
                    text = std::borrow::Cow::Owned(truncate_fallback_text_to_width(
                        &text, fonts, scale, max_width,
                    ));
                    text_width = fallback_text_width_px(&text, fonts, scale);
                }
            }
        }
        let cursor_x = origin.x * surface.width as f32;
        let shrink_offset_y = if matches!(style.overflow, TextOverflow::ShrinkUniform)
            && scale.y < original_scale.y
        {
            (original_scale.y - scale.y) / 2.0
        } else {
            0.0
        };
        let baseline_y =
            origin.y * surface.height as f32 + shrink_offset_y + scaled_primary.ascent();

        self.push_text_line(cursor_x, baseline_y, &text, text_width, scale, style, fonts, surface);

        if let Some(key) = layout_key {
            self.atlas.insert_layout(key, &self.quads[quads_before..]);
        }
    }

    fn push_text_line(
        &mut self,
        origin_x: f32,
        baseline_y: f32,
        text: &str,
        text_width: f32,
        scale: PxScale,
        style: TextStyle,
        fonts: VectorFontSet<'_>,
        surface: SurfaceSize,
    ) {
        let max_width = style.max_width.max(0.0) * surface.width as f32;
        let align_offset = text_align_offset_px(style.align, max_width, text_width);
        let mut cursor_x = origin_x + align_offset;

        for ch in text.chars() {
            let Some((font_id, font)) = fonts.select(ch) else {
                continue;
            };
            let scaled_font = font.as_scaled(scale);
            let glyph_id = font.glyph_id(ch);
            let advance = scaled_font.h_advance(glyph_id);
            if let Some(cached) = self.atlas.cached_vector_glyph(font_id, ch, scale, font) {
                self.quads.push(TextQuad {
                    x: (cursor_x + cached.offset_x) / surface.width as f32,
                    y: (baseline_y + cached.offset_y) / surface.height as f32,
                    width: cached.display_width / surface.width as f32,
                    height: cached.display_height / surface.height as f32,
                    atlas_origin: cached.atlas_origin,
                    glyph_width: cached.width,
                    glyph_height: cached.height,
                    color: style.color,
                });
            }
            cursor_x += advance;
        }
    }

    pub(super) fn finish(
        self,
        command_quad_counts: Vec<usize>,
        command_caret_rects: Vec<Option<RectCommand>>,
    ) -> TextFrame {
        let size = self.atlas.size();
        let instances = encode_text_quads(&self.quads, size.width, size.height);
        TextFrame {
            size,
            #[cfg(test)]
            pixels: Vec::new(),
            dirty_regions: std::mem::take(&mut self.atlas.dirty_regions),
            instances,
            command_quad_counts,
            command_caret_rects,
        }
    }
}
