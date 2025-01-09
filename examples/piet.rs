use kurbo::Affine;
use kurbo::Shape;
use piet::{Color, RenderContext};
use piet_common::kurbo::BezPath;
use piet_common::kurbo::Rect;
use piet_common::Device;

const WIDTH: usize = 1000;
const HEIGHT: usize = 500;

/// Feature "png" needed for save_to_file() and it's disabled by default for optional dependencies
/// cargo run --example mondrian --features png
fn main() {
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH, HEIGHT, 1.0).unwrap();
    let mut rc = bitmap.render_context();

    let background_color = Color::from_hex_str("96C0B7").unwrap();

    rc.fill(
        Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
        &background_color,
    );

    #[cfg(feature = "monospace-font")]
    {
        let mut f = text2v::monospace_font();
        let path_iter = f.render("Hello World", 2000.0);
        let mut combined = BezPath::from_iter(path_iter.0);
        combined.apply_affine(Affine::scale(100.0));
        let bb = combined.bounding_box();
        combined.apply_affine(Affine::translate((
            (WIDTH as f64 / 2.0) - (bb.width() / 2.0),
            (HEIGHT as f64 / 2.0) - (bb.height() / 2.0),
        )));
        rc.stroke(combined, &Color::rgba8(156u8, 1, 188, 255u8), 1.0);
    }

    rc.finish().unwrap();
    std::mem::drop(rc);

    bitmap
        .save_to_file("hello_world.png")
        .expect("file save error");
}
