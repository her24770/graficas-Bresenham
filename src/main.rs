mod framebuffer;
mod life;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const FRAMEBUFFER_WIDTH: usize = 80;
const FRAMEBUFFER_HEIGHT: usize = 60;

fn main() {
    let mut current = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    let mut next = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    current.set_background_color(0x000000);
    next.set_background_color(0x000000);
    current.clear();

    // Patrón de prueba temporal: un glider, para confirmar que las reglas
    // funcionan antes de agregar el resto de los organismos en patterns.rs.
    current.set_current_color(life::ALIVE);
    for (x, y) in [(11, 10), (12, 11), (10, 12), (11, 12), (12, 12)] {
        current.point(x, y);
    }

    let mut window = Window::new(
        "Game of Life",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let frame_delay = Duration::from_millis(120);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        life::step(&current, &mut next);
        std::mem::swap(&mut current, &mut next);

        window
            .update_with_buffer(&current.buffer, FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
