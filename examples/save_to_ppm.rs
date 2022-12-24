use std::fs::File;
use vason::{ppm::encode_canvas, Canvas};

fn main() {
    let mut canvas = Canvas::new(64, 64);
    canvas.clear((180, 255, 100));

    let mut f = File::create("test.ppm").expect("could not create file");
    encode_canvas(&canvas, &mut f).expect("could not write image to file");
}
