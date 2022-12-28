use std::fs::File;
use vason::{ppm::encode_canvas, Canvas, Pen};

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

fn main() {
    let mut canvas = Canvas::new(1024, 1024);
    canvas.clear((15, 15, 35));
    let mut pen = canvas.pen();

    pen.set_position(512.0, 1024.0).set_direction(-90.0);
    tree(&mut pen, 650.0, 15);

    let mut f = File::create("pen.ppm").expect("couldn't create file");
    encode_canvas(&canvas, &mut f).expect("couldn't write image to file");
}
