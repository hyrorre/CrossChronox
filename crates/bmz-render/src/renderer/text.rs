use super::*;

#[derive(Clone)]
pub(super) struct FontFallbackFace {
    pub(super) cache_id: String,
    pub(super) font: FontArc,
}

#[derive(Clone, Default)]
pub(super) struct FontFallbackChain {
    pub(super) faces: Vec<FontFallbackFace>,
}

impl FontFallbackChain {
    pub(super) fn primary(&self) -> Option<&FontFallbackFace> {
        self.faces.first()
    }

    pub(super) fn select(&self, ch: char) -> Option<&FontFallbackFace> {
        self.faces.iter().find(|face| face.font.glyph_id(ch).0 != 0).or_else(|| self.primary())
    }

    #[cfg(test)]
    fn single(font: FontArc) -> Self {
        Self { faces: vec![FontFallbackFace { cache_id: DEFAULT_TEXT_FONT_ID.to_string(), font }] }
    }
}

#[derive(Clone, Copy)]
pub(super) enum VectorFontSet<'a> {
    Single { cache_id: &'a str, font: &'a FontArc },
    Fallback(&'a FontFallbackChain),
}

impl<'a> VectorFontSet<'a> {
    pub(super) fn primary(self) -> Option<(&'a str, &'a FontArc)> {
        match self {
            Self::Single { cache_id, font } => Some((cache_id, font)),
            Self::Fallback(fonts) => {
                fonts.primary().map(|face| (face.cache_id.as_str(), &face.font))
            }
        }
    }

    pub(super) fn select(self, ch: char) -> Option<(&'a str, &'a FontArc)> {
        match self {
            Self::Single { cache_id, font } => Some((cache_id, font)),
            Self::Fallback(fonts) => {
                fonts.select(ch).map(|face| (face.cache_id.as_str(), &face.font))
            }
        }
    }

    pub(super) fn layout_cache_id(self, text: &str) -> String {
        match self {
            Self::Single { cache_id, .. } => cache_id.to_string(),
            Self::Fallback(_) => {
                let mut id = String::from(DEFAULT_TEXT_FONT_ID);
                for ch in text.chars() {
                    let selected = self.select(ch).map(|(id, _)| id).unwrap_or("<missing>");
                    id.push('|');
                    id.push_str(selected);
                }
                id
            }
        }
    }
}

pub(super) fn build_text_frame_with_fallback_cache(
    plan: &DrawPlan,
    default_fonts: &FontFallbackChain,
    fonts: &HashMap<String, FontArc>,
    bitmap_fonts: &HashMap<String, BitmapFont>,
    surface: SurfaceSize,
    atlas: &mut TextAtlasCache,
) -> TextFrame {
    if !surface.is_drawable() {
        return TextFrame::default();
    }

    atlas.begin_frame();
    let mut builder = CachedTextFrameBuilder::new(atlas);
    let mut command_quad_counts = Vec::new();
    let mut command_caret_rects = Vec::new();
    for command in &plan.commands {
        let DrawCommand::Text { origin, text, style, caret, post_scale } = command else {
            continue;
        };
        let quads_before = builder.quads.len();
        if let Some(font_id) = style.font_id.as_deref()
            && let Some(bitmap_font) = bitmap_fonts.get(font_id)
        {
            builder.push_bitmap_text(origin, text, style.clone(), font_id, bitmap_font, surface);
            scale_text_quads(&mut builder.quads[quads_before..], *origin, *post_scale);
            command_caret_rects.push(
                caret
                    .and_then(|caret| {
                        bitmap_text_caret_rect(origin, text, style, bitmap_font, surface, caret)
                    })
                    .map(|caret| scale_text_rect_command(caret, *origin, *post_scale)),
            );
        } else {
            let vector_fonts = style
                .font_id
                .as_deref()
                .and_then(|font_id| {
                    fonts.get(font_id).map(|font| VectorFontSet::Single { cache_id: font_id, font })
                })
                .unwrap_or(VectorFontSet::Fallback(default_fonts));
            builder.push_text(origin, text, style.clone(), vector_fonts, surface);
            scale_text_quads(&mut builder.quads[quads_before..], *origin, *post_scale);
            command_caret_rects.push(
                caret
                    .and_then(|caret| {
                        vector_text_caret_rect_with_fallback(
                            origin,
                            text,
                            style,
                            vector_fonts,
                            surface,
                            caret,
                        )
                    })
                    .map(|caret| scale_text_rect_command(caret, *origin, *post_scale)),
            );
        }
        command_quad_counts.push(builder.quads.len() - quads_before);
    }
    builder.finish(command_quad_counts, command_caret_rects)
}

pub(super) fn scale_text_quads(quads: &mut [TextQuad], origin: Point, scale: Point) {
    if scale == (Point { x: 1.0, y: 1.0 }) {
        return;
    }
    for quad in quads {
        quad.x = origin.x + (quad.x - origin.x) * scale.x;
        quad.y = origin.y + (quad.y - origin.y) * scale.y;
        quad.width *= scale.x;
        quad.height *= scale.y;
    }
}

pub(super) fn scale_text_rect_command(
    mut command: RectCommand,
    origin: Point,
    scale: Point,
) -> RectCommand {
    command.rect.x = origin.x + (command.rect.x - origin.x) * scale.x;
    command.rect.y = origin.y + (command.rect.y - origin.y) * scale.y;
    command.rect.width *= scale.x;
    command.rect.height *= scale.y;
    command
}

#[cfg(test)]
pub(super) fn build_text_frame_with_cache(
    plan: &DrawPlan,
    default_font: &FontArc,
    fonts: &HashMap<String, FontArc>,
    bitmap_fonts: &HashMap<String, BitmapFont>,
    surface: SurfaceSize,
    atlas: &mut TextAtlasCache,
) -> TextFrame {
    build_text_frame_with_fallback_cache(
        plan,
        &FontFallbackChain::single(default_font.clone()),
        fonts,
        bitmap_fonts,
        surface,
        atlas,
    )
}

pub(super) const DEFAULT_TEXT_FONT_ID: &str = "<default>";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TextGlyphKey {
    kind: TextGlyphKind,
    pub(super) font_id: String,
    pub(super) ch: char,
    scale_x_bits: u32,
    scale_y_bits: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum TextGlyphKind {
    Vector,
    Bitmap,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TextLayoutKey {
    kind: TextGlyphKind,
    pub(super) font_id: String,
    pub(super) text: String,
    origin_x_bits: u32,
    origin_y_bits: u32,
    surface_width: u32,
    surface_height: u32,
    style: TextLayoutStyleKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TextLayoutStyleKey {
    size_bits: u32,
    bitmap_size_bits: Option<u32>,
    color: ColorKey,
    align: u8,
    max_width_bits: u32,
    overflow: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ColorKey {
    r_bits: u32,
    g_bits: u32,
    b_bits: u32,
    a_bits: u32,
}

#[derive(Debug, Clone)]
pub(super) struct CachedGlyph {
    pub(super) atlas_origin: (u32, u32),
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) display_width: f32,
    pub(super) display_height: f32,
    pub(super) offset_x: f32,
    pub(super) offset_y: f32,
}

#[derive(Debug)]
pub(super) struct TextLayoutCache {
    pub(super) entries: HashMap<TextLayoutKey, Vec<TextQuad>>,
}

impl TextLayoutCache {
    fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    fn cached(&self, key: &TextLayoutKey) -> Option<&[TextQuad]> {
        self.entries.get(key).map(Vec::as_slice)
    }

    fn insert(&mut self, key: TextLayoutKey, quads: &[TextQuad]) {
        if self.entries.len() >= TEXT_LAYOUT_CACHE_MAX_ENTRIES {
            self.entries.clear();
        }
        self.entries.insert(key, quads.to_vec());
    }

    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Debug)]
pub(super) struct TextAtlasCache {
    width: u32,
    pub(super) pen_x: u32,
    pub(super) pen_y: u32,
    row_height: u32,
    pixels: Vec<u8>,
    pub(super) glyphs: HashMap<TextGlyphKey, CachedGlyph>,
    pub(super) layouts: TextLayoutCache,
    pub(super) dirty_regions: Vec<TextAtlasDirtyRegion>,
}

impl TextAtlasCache {
    pub(super) fn new(width: u32) -> Self {
        Self {
            width,
            pen_x: 0,
            pen_y: 0,
            row_height: 0,
            pixels: Vec::new(),
            glyphs: HashMap::new(),
            layouts: TextLayoutCache::new(),
            dirty_regions: Vec::new(),
        }
    }

    pub(super) fn begin_frame(&mut self) {
        self.dirty_regions.clear();
        // アトラス高さが上限に達したらキャッシュを捨てて作り直す。フレーム描画前に
        // 行うので、このフレームのグリフは新しいアトラスへ再ラスタライズされる。
        if self.atlas_height() >= TEXT_ATLAS_MAX_HEIGHT {
            self.clear();
        }
    }

    fn clear(&mut self) {
        self.pen_x = 0;
        self.pen_y = 0;
        self.row_height = 0;
        self.pixels.clear();
        self.glyphs.clear();
        self.layouts.clear();
        self.dirty_regions.clear();
    }

    pub(super) fn atlas_height(&self) -> u32 {
        (self.pen_y + self.row_height).max(1)
    }

    pub(super) fn size(&self) -> AtlasSize {
        AtlasSize { width: self.width, height: self.atlas_height() }
    }

    pub(super) fn pixels_for_size(&self, size: AtlasSize) -> Vec<u8> {
        let mut pixels = self.pixels.clone();
        pixels.resize((size.width * size.height * 4) as usize, 0);
        pixels
    }

    pub(super) fn cached_layout(&self, key: &TextLayoutKey) -> Option<&[TextQuad]> {
        self.layouts.cached(key)
    }

    pub(super) fn insert_layout(&mut self, key: TextLayoutKey, quads: &[TextQuad]) {
        self.layouts.insert(key, quads);
    }

    pub(super) fn cached_vector_glyph(
        &mut self,
        font_id: &str,
        ch: char,
        scale: PxScale,
        font: &FontArc,
    ) -> Option<CachedGlyph> {
        let key = TextGlyphKey {
            kind: TextGlyphKind::Vector,
            font_id: font_id.to_string(),
            ch,
            scale_x_bits: scale.x.to_bits(),
            scale_y_bits: scale.y.to_bits(),
        };
        if let Some(glyph) = self.glyphs.get(&key) {
            return Some(glyph.clone());
        }

        let raster_scale = PxScale {
            x: scale.x * VECTOR_TEXT_SUPERSAMPLE_SCALE,
            y: scale.y * VECTOR_TEXT_SUPERSAMPLE_SCALE,
        };
        let scaled_font = font.as_scaled(raster_scale);
        let baseline_y = scaled_font.ascent();
        let glyph =
            Glyph { id: font.glyph_id(ch), scale: raster_scale, position: point(0.0, baseline_y) };
        let outlined = font.outline_glyph(glyph)?;
        let bounds = outlined.px_bounds();
        let width = bounds.width().ceil().max(0.0) as u32;
        let height = bounds.height().ceil().max(0.0) as u32;
        if width == 0 || height == 0 {
            return None;
        }

        let mut pixels = vec![0; (width * height * 4) as usize];
        outlined.draw(|x, y, coverage| {
            let index = ((y * width + x) * 4) as usize;
            let coverage_u8 = (coverage * 255.0).clamp(0.0, 255.0) as u8;
            if let Some(pixel) = pixels.get_mut(index..index + 4)
                && coverage_u8 >= pixel[3]
            {
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
                pixel[3] = coverage_u8;
            }
        });

        Some(self.insert_glyph_pixels(
            key,
            width,
            height,
            width as f32 / VECTOR_TEXT_SUPERSAMPLE_SCALE,
            height as f32 / VECTOR_TEXT_SUPERSAMPLE_SCALE,
            bounds.min.x / VECTOR_TEXT_SUPERSAMPLE_SCALE,
            (bounds.min.y - baseline_y) / VECTOR_TEXT_SUPERSAMPLE_SCALE,
            pixels,
        ))
    }

    pub(super) fn cached_bitmap_glyph(
        &mut self,
        font_id: &str,
        ch: char,
        glyph: crate::bitmap_font::BitmapFontGlyph,
        page: &crate::bitmap_font::BitmapFontPage,
        font: &BitmapFont,
        scale: PxScale,
    ) -> CachedGlyph {
        let key = TextGlyphKey {
            kind: TextGlyphKind::Bitmap,
            font_id: font_id.to_string(),
            ch,
            scale_x_bits: scale.x.to_bits(),
            scale_y_bits: scale.y.to_bits(),
        };
        if let Some(glyph) = self.glyphs.get(&key) {
            return glyph.clone();
        }

        let width = (glyph.width as f32 * scale.x).ceil().max(1.0) as u32;
        let height = (glyph.height as f32 * scale.y).ceil().max(1.0) as u32;
        let pixels = rasterized_bitmap_glyph_pixels(glyph, page, scale, width, height);

        self.insert_glyph_pixels(
            key,
            width,
            height,
            width as f32,
            height as f32,
            glyph.xoffset as f32 * scale.x,
            (glyph.yoffset as f32 - font.ascent) * scale.y,
            pixels,
        )
    }

    fn insert_glyph_pixels(
        &mut self,
        key: TextGlyphKey,
        width: u32,
        height: u32,
        display_width: f32,
        display_height: f32,
        offset_x: f32,
        offset_y: f32,
        pixels: Vec<u8>,
    ) -> CachedGlyph {
        let atlas_origin = self.reserve(width, height);
        self.blit_region(atlas_origin, width, height, &pixels);
        self.dirty_regions.push(TextAtlasDirtyRegion {
            origin: atlas_origin,
            size: AtlasSize { width, height },
            pixels,
        });
        let cached = CachedGlyph {
            atlas_origin,
            width,
            height,
            display_width,
            display_height,
            offset_x,
            offset_y,
        };
        self.glyphs.insert(key, cached.clone());
        cached
    }

    pub(super) fn reserve(&mut self, glyph_width: u32, glyph_height: u32) -> (u32, u32) {
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

    fn blit_region(&mut self, atlas_origin: (u32, u32), width: u32, height: u32, pixels: &[u8]) {
        for y in 0..height {
            let dst_start = (((atlas_origin.1 + y) * self.width + atlas_origin.0) * 4) as usize;
            let src_start = (y * width * 4) as usize;
            let len = (width * 4) as usize;
            if let (Some(dst), Some(src)) = (
                self.pixels.get_mut(dst_start..dst_start + len),
                pixels.get(src_start..src_start + len),
            ) {
                dst.copy_from_slice(src);
            }
        }
    }
}

pub(super) fn text_layout_key(
    kind: TextGlyphKind,
    font_id: &str,
    origin: &Point,
    text: &str,
    style: &TextStyle,
    surface: SurfaceSize,
) -> Option<TextLayoutKey> {
    if style.wrapping || style.outline.is_some() || style.shadow.is_some() {
        return None;
    }
    Some(TextLayoutKey {
        kind,
        font_id: font_id.to_string(),
        text: text.to_string(),
        origin_x_bits: origin.x.to_bits(),
        origin_y_bits: origin.y.to_bits(),
        surface_width: surface.width,
        surface_height: surface.height,
        style: TextLayoutStyleKey {
            size_bits: style.size.to_bits(),
            bitmap_size_bits: style.bitmap_size.map(f32::to_bits),
            color: ColorKey {
                r_bits: style.color.r.to_bits(),
                g_bits: style.color.g.to_bits(),
                b_bits: style.color.b.to_bits(),
                a_bits: style.color.a.to_bits(),
            },
            align: text_align_key(style.align),
            max_width_bits: style.max_width.to_bits(),
            overflow: text_overflow_key(style.overflow),
        },
    })
}

pub(super) fn text_align_key(align: TextAlign) -> u8 {
    match align {
        TextAlign::Left => 0,
        TextAlign::Center => 1,
        TextAlign::Right => 2,
    }
}

pub(super) fn text_overflow_key(overflow: TextOverflow) -> u8 {
    match overflow {
        TextOverflow::Overflow => 0,
        TextOverflow::Shrink => 1,
        TextOverflow::Truncate => 2,
        TextOverflow::ShrinkUniform => 3,
    }
}
