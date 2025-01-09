# text2v
Library to convert text into vector primitives from [kurbo](https://github.com/linebender/kurbo) crate.
This project is a reimplementation of [fonterator](https://github.com/ardaku/fonterator) that replaces the underlying [footile](https://github.com/DougLau/footile) graphics primitives with equivalent functionality from the [kurbo](https://github.com/linebender/kurbo) crate.
Main motivation for this project is to be able to produce [rough]() styled text.

## Example

```rust
use text2v::{Font, Text};

fn main() {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 500;
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH, HEIGHT, 1.0).unwrap();
    let mut rc = bitmap.render_context();

    let background_color = Color::from_hex_str("96C0B7").unwrap();

    rc.fill(
        Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
        &background_color,
    );

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

    rc.finish().unwrap();
    std::mem::drop(rc);

    bitmap
        .save_to_file("hello_world.png")
        .expect("file save error");
}
```

## License
This project is licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
