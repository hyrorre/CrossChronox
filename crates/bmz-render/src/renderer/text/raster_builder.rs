use super::*;

#[cfg(test)]
pub(super) struct TextAtlasBuilder {
    width: u32,
    pen_x: u32,
    pen_y: u32,
    row_height: u32,
    pixels: Vec<u8>,
    pub(super) quads: Vec<TextQuad>,
}

#[derive(Debug, Clone)]
pub(super) struct TextQuad {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) atlas_origin: (u32, u32),
    pub(super) glyph_width: u32,
    pub(super) glyph_height: u32,
    pub(super) color: Color,
}

#[cfg(test)]
impl TextAtlasBuilder {
    pub(super) fn new(width: u32) -> Self {
        Self { width, pen_x: 0, pen_y: 0, row_height: 0, pixels: Vec::new(), quads: Vec::new() }
    }

    pub(super) fn push_bitmap_text(
        &mut self,
        origin: &Point,
        text: &str,
        style: TextStyle,
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
            self.push_bitmap_text(&shadow_origin, text, shadow_style, font, surface);
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
                self.push_bitmap_text(&outline_origin, text, outline_style.clone(), font, surface);
            }
        }

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
            if glyph.width > 0 && glyph.height > 0 {
                let Some(page) = font.pages.get(&glyph.page) else {
                    cursor_x += glyph.xadvance as f32 * scale.x;
                    continue;
                };
                let glyph_width = (glyph.width as f32 * scale.x).ceil().max(1.0) as u32;
                let glyph_height = (glyph.height as f32 * scale.y).ceil().max(1.0) as u32;
                let atlas_origin = self.reserve(glyph_width, glyph_height);
                self.blit_bitmap_glyph(
                    atlas_origin,
                    glyph_width,
                    glyph_height,
                    *glyph,
                    page,
                    scale,
                );
                let x = (cursor_x + glyph.xoffset as f32 * scale.x) / surface.width as f32;
                let y = (text_top_y + (glyph.yoffset as f32 - font.ascent) * scale.y)
                    / surface.height as f32;
                self.quads.push(TextQuad {
                    x,
                    y,
                    width: glyph_width as f32 / surface.width as f32,
                    height: glyph_height as f32 / surface.height as f32,
                    atlas_origin,
                    glyph_width,
                    glyph_height,
                    color: style.color,
                });
            }
            cursor_x += glyph.xadvance as f32 * scale.x;
        }
    }

    fn blit_bitmap_glyph(
        &mut self,
        atlas_origin: (u32, u32),
        glyph_width: u32,
        glyph_height: u32,
        glyph: crate::bitmap_font::BitmapFontGlyph,
        page: &crate::bitmap_font::BitmapFontPage,
        scale: PxScale,
    ) {
        let pixels = rasterized_bitmap_glyph_pixels(glyph, page, scale, glyph_width, glyph_height);
        for dst_y in 0..glyph_height {
            for dst_x in 0..glyph_width {
                let src_index = ((dst_y * glyph_width + dst_x) * 4) as usize;
                let Some(src) = pixels.get(src_index..src_index + 4) else {
                    continue;
                };
                let dst_index =
                    (((atlas_origin.1 + dst_y) * self.width + atlas_origin.0 + dst_x) * 4) as usize;
                let Some(dst) = self.pixels.get_mut(dst_index..dst_index + 4) else {
                    continue;
                };
                // ビットマップフォントは RGBA を保持して描画したい (色付きグリフ対応)。
                // 同じアトラス位置に重ねたい場合は alpha が大きい方を採用。
                if src[3] >= dst[3] {
                    dst.copy_from_slice(src);
                }
            }
        }
    }

    pub(super) fn push_text(
        &mut self,
        origin: &Point,
        text: &str,
        style: TextStyle,
        font: &FontArc,
        surface: SurfaceSize,
    ) {
        if let Some(shadow) = style.shadow.filter(|shadow| shadow.color.a > 0.0) {
            let mut shadow_style = style.clone();
            shadow_style.color = shadow.color;
            shadow_style.outline = None;
            shadow_style.shadow = None;
            let shadow_origin =
                Point { x: origin.x + shadow.offset.x, y: origin.y + shadow.offset.y };
            self.push_text(&shadow_origin, text, shadow_style, font, surface);
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
                self.push_text(&outline_origin, text, outline_style.clone(), font, surface);
            }
        }

        let px_size = (style.size * surface.height as f32).max(1.0);
        let max_width = style.max_width.max(0.0) * surface.width as f32;
        let mut text = std::borrow::Cow::Borrowed(text);
        let mut scale = PxScale::from(px_size);
        let original_scale = scale;
        let mut scaled_font = font.as_scaled(scale);
        let mut text_width = text_width_px(&text, font, &scaled_font);
        if style.wrapping && max_width > 0.0 {
            let lines = wrap_text_to_width(&text, font, &scaled_font, max_width);
            let line_height = (scaled_font.ascent() - scaled_font.descent()
                + scaled_font.line_gap())
            .max(px_size);
            let origin_x = origin.x * surface.width as f32;
            let first_baseline_y = origin.y * surface.height as f32 + scaled_font.ascent();
            for (index, line) in lines.iter().enumerate() {
                let line_width = text_width_px(line, font, &scaled_font);
                let baseline_y = first_baseline_y + line_height * index as f32;
                self.push_text_line(
                    origin_x,
                    baseline_y,
                    line,
                    line_width,
                    scale,
                    style.clone(),
                    font,
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
                    scaled_font = font.as_scaled(scale);
                    text_width = text_width_px(&text, font, &scaled_font);
                }
                TextOverflow::ShrinkUniform => {
                    scale = PxScale::from((scale.x * max_width / text_width).max(1.0));
                    scaled_font = font.as_scaled(scale);
                    text_width = text_width_px(&text, font, &scaled_font);
                }
                TextOverflow::Truncate => {
                    text = std::borrow::Cow::Owned(truncate_text_to_width(
                        &text,
                        font,
                        &scaled_font,
                        max_width,
                    ));
                    text_width = text_width_px(&text, font, &scaled_font);
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
        let baseline_y = origin.y * surface.height as f32 + shrink_offset_y + scaled_font.ascent();

        self.push_text_line(cursor_x, baseline_y, &text, text_width, scale, style, font, surface);
    }

    fn push_text_line(
        &mut self,
        origin_x: f32,
        baseline_y: f32,
        text: &str,
        text_width: f32,
        scale: PxScale,
        style: TextStyle,
        font: &FontArc,
        surface: SurfaceSize,
    ) {
        let scaled_font = font.as_scaled(scale);
        let max_width = style.max_width.max(0.0) * surface.width as f32;
        let align_offset = text_align_offset_px(style.align, max_width, text_width);
        let mut cursor_x = origin_x + align_offset;

        for ch in text.chars() {
            let glyph_id = font.glyph_id(ch);
            let advance = scaled_font.h_advance(glyph_id);
            let glyph = Glyph { id: glyph_id, scale, position: point(cursor_x, baseline_y) };
            if let Some(outlined) = font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let glyph_width = bounds.width().ceil().max(0.0) as u32;
                let glyph_height = bounds.height().ceil().max(0.0) as u32;
                if glyph_width > 0 && glyph_height > 0 {
                    let atlas_origin = self.reserve(glyph_width, glyph_height);
                    outlined.draw(|x, y, coverage| {
                        let dst_x = atlas_origin.0 + x;
                        let dst_y = atlas_origin.1 + y;
                        let index = ((dst_y * self.width + dst_x) * 4) as usize;
                        let coverage_u8 = (coverage * 255.0).clamp(0.0, 255.0) as u8;
                        if let Some(pixel) = self.pixels.get_mut(index..index + 4) {
                            // TTF グリフは白 RGB に coverage を alpha として書く。
                            // 既存値より coverage が大きい場合のみ上書き。
                            if coverage_u8 >= pixel[3] {
                                pixel[0] = 255;
                                pixel[1] = 255;
                                pixel[2] = 255;
                                pixel[3] = coverage_u8;
                            }
                        }
                    });
                    self.quads.push(TextQuad {
                        x: bounds.min.x / surface.width as f32,
                        y: bounds.min.y / surface.height as f32,
                        width: glyph_width as f32 / surface.width as f32,
                        height: glyph_height as f32 / surface.height as f32,
                        atlas_origin,
                        glyph_width,
                        glyph_height,
                        color: style.color,
                    });
                }
            }
            cursor_x += advance;
        }
    }

    fn reserve(&mut self, glyph_width: u32, glyph_height: u32) -> (u32, u32) {
        let padded_width = glyph_width + TEXT_ATLAS_PADDING * 2;
        let padded_height = glyph_height + TEXT_ATLAS_PADDING * 2;
        if self.pen_x + padded_width > self.width {
            self.pen_x = 0;
            self.pen_y += self.row_height;
            self.row_height = 0;
        }

        let origin = (self.pen_x + TEXT_ATLAS_PADDING, self.pen_y + TEXT_ATLAS_PADDING);
        self.pen_x += padded_width;
        self.row_height = self.row_height.max(padded_height);
        self.ensure_height(self.pen_y + self.row_height);
        origin
    }

    fn ensure_height(&mut self, height: u32) {
        let needed = (self.width * height * 4) as usize;
        if self.pixels.len() < needed {
            self.pixels.resize(needed, 0);
        }
    }

    fn atlas_height(&self) -> u32 {
        (self.pen_y + self.row_height).max(1)
    }

    pub(super) fn finish(mut self) -> TextFrame {
        let height = self.atlas_height();
        self.pixels.resize((self.width * height * 4) as usize, 0);
        let instances = encode_text_quads(&self.quads, self.width, height);
        TextFrame {
            size: AtlasSize { width: self.width, height },
            #[cfg(test)]
            pixels: self.pixels,
            dirty_regions: Vec::new(),
            instances,
            command_quad_counts: Vec::new(),
            command_caret_rects: Vec::new(),
        }
    }
}
