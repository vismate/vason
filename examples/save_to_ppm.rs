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
