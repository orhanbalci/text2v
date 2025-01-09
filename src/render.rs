use kurbo::BezPath;
use rustybuzz::Face;

/// Build a path
pub(crate) fn build_path(
    path_buffer: &mut BezPath,
    face: &Face<'_>,
    glyph_x: f32,
    glyph_y: f32,
    glyph_id: u16,
) {
}
