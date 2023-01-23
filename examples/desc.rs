use std::fs::File;
use vason::{
    ppm::encode_canvas,
    shape::{Rectangle, Triangle},
    Canvas, Color,
};

fn main() {
    let mut buffer = [0u32; 256 * 256];
    let mut canvas = Canvas::new(&mut buffer, 256, 256);

    let mut rect = Rectangle::new(50, 50, 100, 100);
    rect.set_fill_color(Some(Color::RED))
        .set_outline_thickness(2)
        .set_outline_color(Some(Color::BLUE));

    let mut triangle = Triangle::new(100, 100, 200, 200, 100, 200);
    triangle
        .set_fill_color(Some(Color::INDIGO))
        .set_outline_thickness(3)
        .set_outline_color(Some(Color::GOLD));

    canvas.draw(&rect);
    canvas.draw(&triangle);

    let mut f = File::create("desc.ppm").expect("could not create file");

    encode_canvas(&canvas, &mut f).expect("could not write image to file");
}
