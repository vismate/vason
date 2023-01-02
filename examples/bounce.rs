/// It's a very good idea to run this in release mode.
/// Optimizations go a long way...
use minifb::{Key, Window, WindowOptions};
use vason::{Canvas, Color};

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;

fn main() {
    let mut buffer = vec![0u32; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut canvas = Canvas::new(&mut buffer, SCREEN_WIDTH, SCREEN_HEIGHT);

    let width = SCREEN_WIDTH as i32;
    let height = SCREEN_HEIGHT as i32;

    let r = 75;
    let border_inner = 25;
    let mut px = width / 2;
    let mut py = height - 2 * r;
    let mut vx = 3;
    let mut vy = 6;

    let mut window = Window::new(
        "Bounce Example - ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // collide with walls

        if px - r < border_inner {
            px = r + border_inner;
            vx = -vx;
        }

        if px + r > width - border_inner {
            px = width - border_inner - r;
            vx = -vx;
        }

        if py - r < border_inner {
            py = r + border_inner;
            vy = -vy;
        }

        if py + r > height - border_inner {
            py = height - border_inner - r;
            vy = -vy;
        }

        // update position
        px += vx;
        py += vy;

        // draw frame
        canvas.clear(Color::LIGHT_GRAY);

        // border
        canvas.thick_outline_rect(0, 0, width, height, border_inner * 2, Color::GRAY);

        //ball
        canvas.fill_circle(px, py, r, Color::rgb(255, 100, 30));
        canvas.thick_outline_circle(px, py, r / 3 * 2, 12, Color::GOLD);

        window
            .update_with_buffer(canvas.buffer(), SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
