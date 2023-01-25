# vason - simple 2D rasterizer

**WARNING: This crate is in very early stages. Anything can change anytime. Use at your own risk.**

This crate enables you to render simple 2D shapes to a buffer of pixels.
After creating a Canvas from a buffer, you have access to methods to fill in, or draw the outline of shapes.

This crate has **no runtime dependencies.**

A ppm module is included that lets you save your buffer as an image (that can be displayed by some major image viewers).

The crate also works well together with libraries such as [minifb](https://crates.io/crates/minifb), thus you can even use it for small games / demos / visualizations.

## Current and planned features:
- [x] basic shape rendering:
  - [x] fill_rect, outline_rect, thick_outline_rect
  - [x] fill_circle, outline_circle,
  - [x] fill_ellipse, outline_ellipse,
  - [x] line, hline, vline, thick_hline, thick_vline
- [x] Save buffer to a primitive image format (ppm)
- [ ] more shapes:
  - [x] fill_triangle, outline_triangle, thick_outline_triangle
  - [x] thick_outline_circle
  - [x] thick_outline_ellipse
  - [x] thick_line
  - [ ] bezier_curve
- [x] flood fill
- [ ] copy regions over from other buffer (sprites)
- [x] Pen-API: ["Turtle Geometry"](https://people.eecs.berkeley.edu/~bh/v1ch10/turtle.html)
- [x] Shape-API: A higher level helper API that can make your code more readable (but a tad less efficient).
- [ ] alpha compositing (transparency)
- [ ] built-in monospaced font rendering
- further optimizations...
- and more...


## Example
```rust
use std::fs::File;
use vason::{ppm::encode_canvas, Canvas, Color};

fn main() {
    let mut buffer = vec![0u32; 256*256];
    let mut canvas = Canvas::new(&mut buffer, 256, 256);
    canvas.clear((180, 255, 100));
    canvas.fill_rect(80, 40, 128, 192, Color::GREEN);
    canvas.fill_circle(-40, -40, 128, Color::BLUE);
    canvas.outline_circle(-40, -40, 178, Color::RED);
    canvas.line(256, 0, 0, 256, Color::MAGENTA);

    let mut f = File::create("test.ppm").expect("could not create file");
    encode_canvas(&canvas, &mut f).expect("could not write image to file");
}
```

You may use the Pen-API to easily draw to the buffer images such as this:
```rust
use std::fs::File;
use vason::{ppm::encode_canvas, Canvas, Color, Pen};

fn tree(pen: &mut Pen, size: f32, depth: i32) {
    let state = pen.get_state();
    let green = ((20 - depth) * 15).clamp(0, u8::MAX as i32) as u8;
    pen.set_color((0, green, 0));

    if depth <= 0 || size < 5.0 {
        pen.forward(size).backward(size);
        return;
    }
    pen.forward(size / 3.0).turn_left(30.0);
    tree(pen, (size * 2.0) / 3.0, depth - 1);
    pen.turn_right(30.0).forward(size / 6.0).turn_right(25.0);
    tree(pen, size / 2.0, depth - 1);
    pen.turn_left(25.0).forward(size / 3.0).turn_right(25.0);
    tree(pen, size / 2.0, depth - 1);
    pen.turn_left(25.0).forward(size / 6.0).set_state(state);
}

fn sun(pen: &mut Pen, scale: f32) {
    pen.repeat(18, |pen| {
        pen.forward(0.5 * scale)
            .turn_right(150.0)
            .forward(0.6 * scale)
            .turn_right(100.0)
            .forward(0.3 * scale)
            .turn_right(90.0);
    });
}

fn main() {
    let mut buffer = vec![0u32; 1024 * 1024];
    let mut canvas = Canvas::new(&mut buffer, 1024, 1024);
    canvas.clear((15, 15, 35));
    let mut pen = canvas.pen();

    pen.set_position(512.0, 1024.0).set_direction(-90.0);
    tree(&mut pen, 650.0, 15);

    pen.set_color(Color::YELLOW).set_position(80.0, 120.0);
    sun(&mut pen, 175.0);

    let mut f = File::create("pen.ppm").expect("couldn't create file");
    encode_canvas(&canvas, &mut f).expect("couldn't write image to file");
}
```

This piece of code produces the following image:
![rendered image](https://imgur.com/xo5n3sF.jpg)