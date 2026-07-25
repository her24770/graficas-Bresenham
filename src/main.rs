mod framebuffer;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const FRAMEBUFFER_WIDTH: usize = 80;
const FRAMEBUFFER_HEIGHT: usize = 60;

fn main() {
    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let mut window = Window::new(
        "Game of Life",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&framebuffer.buffer, FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT)
            .unwrap();
    }
}
