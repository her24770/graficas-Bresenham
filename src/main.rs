mod framebuffer;
mod life;
mod patterns;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 800;
const FRAMEBUFFER_WIDTH: usize = 100;
const FRAMEBUFFER_HEIGHT: usize = 100;

fn main() {
    let mut current = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    let mut next = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);
    current.set_background_color(0x000000);
    next.set_background_color(0x000000);
    current.clear();

    place_initial_pattern(&mut current);

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

fn place_initial_pattern(fb: &mut Framebuffer) {
    // Gun: dispara gliders sin parar hacia la esquina inferior derecha.
    patterns::gosper_glider_gun(fb, 2, 5);

    // Oscillator grande.
    patterns::pulsar(fb, 50, 5);

    // Spaceships.
    patterns::lightweight_spaceship(fb, 75, 8);
    patterns::glider(fb, 10, 65);
    patterns::glider(fb, 60, 70);

    // Osciladores chicos.
    patterns::blinker(fb, 10, 30);
    patterns::toad(fb, 20, 30);
    patterns::beacon(fb, 30, 30);

    // Still lifes.
    patterns::block(fb, 45, 30);
    patterns::beehive(fb, 55, 30);
    patterns::loaf(fb, 65, 30);
    patterns::boat(fb, 78, 30);
    patterns::tub(fb, 88, 30);
}
