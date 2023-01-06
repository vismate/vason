use minifb::{Key, Window, WindowOptions};
use vason::{Canvas, Color};

const WIDTH: usize = 1366;
const HEIGHT: usize = 768;

//TODO: Create a better example/benchmark
fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut canvas = Canvas::new(&mut buffer, WIDTH, HEIGHT);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // TODO: Refactor these variable names
    let mut cx = 0;
    let mut ry = 0;
    let mut i = 0;
    let mut id = 1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = std::time::Instant::now();
        canvas.clear((130, 233, 120));
        for y in 0..156 {
            canvas.line(0, (y + 15) * 5, WIDTH as i32, y * 5, 100u8);
        }

        for x in 0..276 {
            canvas.line(x * 5, 0, (x + 15) * 5, HEIGHT as i32, 100u8);
        }

        for y in 0..7 {
            for x in 0..i {
                canvas.fill_rect(x * 21, 34 + 101 * y, 20, 100, (22, 45, 185));
            }
        }
        for y in 0..28 {
            for x in 0..i {
                canvas.fill_circle(WIDTH as i32 - x * 21, 34 + 25 * y, 10, (185, 45, 185));
            }
        }
        canvas.fill_circle(cx - 275, 300, 275, Color::RED);
        cx = (cx + 1) % (WIDTH + 275 * 2) as i32;
        canvas.fill_rect(500, ry - 350, 800, 350, Color::rgb(120, 130, 233));
        ry = (ry + 1) % (HEIGHT + 350) as i32;

        canvas.line(0, 0, WIDTH as i32, HEIGHT as i32, 200u8);
        canvas.line(WIDTH as i32, 0, 0, HEIGHT as i32, 200u8);

        i = i + id;

        if i > 65 {
            id = -1;
        }
        if i < 0 {
            id = 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(canvas.buffer(), WIDTH, HEIGHT)
            .unwrap();
        if i > 65 {
            println!(
                "{:?}",
                std::time::Instant::now().duration_since(frame_start)
            );
        }
    }
}
