# vason - simple 2D rasterizer

**WARNING: This crate is in very early stages. Anything could changes anytime. Use at your own risk.**

This crate enables you to render simple 2D shapes to a buffer of pixels.
After creating a Canvas, you have access to methods to fill in, or draw the outline of shapes.

A ppm module is included that lets you save your buffer as an image (that can be displayed by some major image viewers).
The crate also works well together with minifb, thus you can even use it for small games / demos / visualizations.

## Current and planned features:
- [x] basic shape rendering:
  - [x] fill_rect, outline_rect, thick_outline_rect
  - [x] fill_circle, outline_circle,
  - [x] fill_ellipse, outline_ellipse,
  - [x] line, hline, vline, thick_hline, thick_vline
- [x] Save buffer to a primitive image format (ppm)
- [ ] more shapes:
  - [ ] fill_triangle, outline_triangle, thick_outline_triangle
  - [ ] thick_outline_circle
  - [ ] thick_outline_ellipse
  - [ ] thick_line
  - [ ] bezier_curve
- [ ] copy regions over from other buffer (sprites)
- [ ] Pen-API: ["Turtle Geometry"](https://people.eecs.berkeley.edu/~bh/v1ch10/turtle.html)
- [ ] alpha compositing (transparency)
- [ ] built-in monospaced font rendering
- further optimizations...
- and more...


## Example
```rust
use std::fs::File;
use vason::{ppm::encode_canvas, Canvas, Color};

fn main() {
    let mut canvas = Canvas::new(256, 256);
    canvas.clear((180, 255, 100));
    canvas.fill_rect(80, 40, 128, 192, Color::GREEN);
    canvas.fill_circle(-40, -40, 128, Color::BLUE);
    canvas.outline_circle(-40, -40, 178, Color::RED);
    canvas.line(256, 0, 0, 256, Color::MAGENTA);

    let mut f = File::create("test.ppm").expect("could not create file");
    encode_canvas(&canvas, &mut f).expect("could not write image to file");
}
```