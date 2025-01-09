//! Fonterator's text shaping with rustybuzz

use crate::render;
use kurbo::{BezPath, PathEl};
use rustybuzz::{Face, GlyphBuffer, UnicodeBuffer};

fn glyph_buffer_with_text(face: &Face<'_>, glyph_buffer: GlyphBuffer, text: &str) -> GlyphBuffer {
    let mut unicode_buffer = glyph_buffer.clear();
    unicode_buffer.push_str(text);
    rustybuzz::shape(&face, &[], unicode_buffer)
}

fn new_glyph_buffer(face: &Face<'_>) -> GlyphBuffer {
    let unicode_buffer = UnicodeBuffer::new();
    let glyph_buffer = rustybuzz::shape(&face, &[], unicode_buffer);

    glyph_buffer
}

fn to_f32(scale: f32, input: i32) -> f32 {
    input as f32 * scale
}

/// A TTF font
pub struct Font<'a> {
    face: Face<'a>,
    glyph_buffer: GlyphBuffer,
    path_buffer: BezPath,
}

impl<'a> Font<'a> {
    /// Load a font from a TTF file
    pub fn new(ttf: &'a [u8]) -> Result<Self, ()> {
        let face = Face::from_slice(ttf, 0).ok_or(())?;
        let glyph_buffer = new_glyph_buffer(&face);
        let path_buffer = BezPath::new();

        Ok(Self {
            face,
            glyph_buffer,
            path_buffer,
        })
    }

    /// Render text including shaping and layout
    ///
    /// # Parameters
    ///  - `text`: The UTF-8 text to render
    ///  - `row`: The available rendering width in ems
    pub(crate) fn render<'b: 'a>(
        &'b mut self,
        text: &mut &str,
        _row: f32,
    ) -> impl Iterator<Item = PathEl> + 'b + 'a {
        // FIXME: Lookahead
        let consumed = (*text).len();
        let piece = &(*text)[..consumed];

        let mut glyph_buffer = new_glyph_buffer(&self.face);
        std::mem::swap(&mut glyph_buffer, &mut self.glyph_buffer);
        self.glyph_buffer = glyph_buffer_with_text(&self.face, glyph_buffer, piece);

        *text = &text[consumed..];

        Render {
            font: self,
            glyph_index: 0,
            advance_x: 0.0,
            advance_y: 0.0,
        }
    }
}

struct Render<'a, 'b> {
    font: &'b mut Font<'a>,
    glyph_index: usize,
    advance_x: f32,
    advance_y: f32,
}

impl Iterator for Render<'_, '_> {
    type Item = PathEl;

    fn next(&mut self) -> Option<Self::Item> {
        // Queued path operations
        if let Some(path_op) = self.font.path_buffer.pop() {
            return Some(path_op);
        }

        // Check the current glyph
        if self.glyph_index >= self.font.glyph_buffer.len() {
            return None;
        }

        // Build current glyph path
        let units_per_em = self.font.face.units_per_em();
        let scale = (units_per_em as f32).recip();

        let glyph_infos = self.font.glyph_buffer.glyph_infos();
        let glyph_info = glyph_infos[self.glyph_index];
        let glyph_positions = self.font.glyph_buffer.glyph_positions();
        let glyph_position = glyph_positions[self.glyph_index];

        let glyph_id = glyph_info.glyph_id as u16;
        let _cluster_index = glyph_info.cluster;
        let _clustered = glyph_info.unsafe_to_break();

        self.advance_x += to_f32(scale, glyph_position.x_advance);
        self.advance_y += to_f32(scale, glyph_position.y_advance);

        let glyph_x = self.advance_x + to_f32(scale, glyph_position.x_offset);
        let glyph_y = self.advance_y + to_f32(scale, glyph_position.y_offset);

        render::build_path(
            &mut self.font.path_buffer,
            &self.font.face,
            glyph_x,
            glyph_y,
            glyph_id,
        );

        self.glyph_index += 1;

        // Tail call recursion
        self.next()
    }
}
