use std::fs::File;
use vason::{ppm::encode_canvas, Canvas, Color};

fn main() {
    let mut canvas = Canvas::new(64, 64);
    canvas.clear((180, 255, 100));
    canvas.fill_rect(20, 10, 32, 48, Color::GREEN);
    canvas.fill_circle(-10, -10, 32, Color::BLUE);
    canvas.line(64, 0, 0, 64, Color::MAGENTA);

    let mut f = File::create("test.ppm").expect("could not create file");
    encode_canvas(&canvas, &mut f).expect("could not write image to file");
}
