use super::*;

#[test]
fn gpu_texture_validation_rejects_oversized_images_before_allocation() {
    assert!(validate_rgba_texture_for_device(4, 5, 1, &[0; 20]).is_err());
    assert!(validate_rgba_texture_for_device(4, 1, 5, &[0; 20]).is_err());
    assert!(validate_rgba_texture_for_device(4, 4, 4, &[0; 64]).is_ok());
    assert!(validate_rgba_texture_for_device(4, 0, 1, &[]).is_err());
    assert!(validate_rgba_texture_for_device(4, 1, 1, &[0; 3]).is_err());
}

#[test]
fn renderer_queues_texture_assets_before_surface_attach() {
    let mut renderer = Renderer::default();
    let asset = crate::assets::RgbaImageAsset { width: 1, height: 1, pixels: vec![255, 0, 0, 255] };

    renderer.upsert_image_asset(crate::plan::TextureId(9), &asset).unwrap();

    assert_eq!(renderer.pending_textures.len(), 1);
    assert_eq!(renderer.pending_textures[0].id, crate::plan::TextureId(9));
}

#[test]
fn installing_vector_font_replaces_stale_bitmap_font_with_same_id() {
    let Some(font) = load_default_font() else { return };
    let mut renderer = Renderer::default();

    renderer.insert_bitmap_font_entry("play:0".to_string(), test_bitmap_font());
    renderer.insert_vector_font("play:0".to_string(), font);

    assert!(renderer.fonts.contains_key("play:0"));
    assert!(!renderer.bitmap_fonts.contains_key("play:0"));
}

#[test]
fn installing_bitmap_font_replaces_stale_vector_font_with_same_id() {
    let Some(font) = load_default_font() else { return };
    let mut renderer = Renderer::default();

    renderer.insert_vector_font("play:0".to_string(), font);
    renderer.insert_bitmap_font_entry("play:0".to_string(), test_bitmap_font());

    assert!(renderer.bitmap_fonts.contains_key("play:0"));
    assert!(!renderer.fonts.contains_key("play:0"));
}
