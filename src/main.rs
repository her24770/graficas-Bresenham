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
        render(&mut current, &mut next, &mut window);
        std::thread::sleep(frame_delay);
    }
}

// Un turno completo: calcula el siguiente estado, intercambia los buffers
// y presenta el resultado en la ventana.
fn render(current: &mut Framebuffer, next: &mut Framebuffer, window: &mut Window) {
    life::step(current, next);
    std::mem::swap(current, next);

    window
        .update_with_buffer(&current.buffer, FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT)
        .unwrap();
}

fn place_initial_pattern(fb: &mut Framebuffer) {
    // --- Franja superior: estructuras grandes ---
    patterns::gosper_glider_gun(fb, 2, 2); // dispara gliders sin parar
    patterns::pulsar(fb, 42, 2);
    patterns::heavyweight_spaceship(fb, 60, 4);
    patterns::middleweight_spaceship(fb, 70, 4);
    patterns::pentadecathlon(fb, 80, 6);

    // --- Franja media: osciladores chicos, still lifes y naves ---
    patterns::blinker(fb, 5, 32);
    patterns::toad(fb, 12, 32);
    patterns::beacon(fb, 20, 32);
    patterns::block(fb, 28, 32);
    patterns::beehive(fb, 34, 32);
    patterns::loaf(fb, 42, 32);
    patterns::boat(fb, 50, 32);
    patterns::tub(fb, 58, 32);
    patterns::r_pentomino(fb, 66, 32);
    patterns::lightweight_spaceship(fb, 74, 32);
    patterns::acorn(fb, 85, 32);

    // --- Franja inferior: methuselahs y gliders sueltos ---
    patterns::diehard(fb, 10, 55);
    patterns::glider(fb, 30, 60);
    patterns::glider(fb, 55, 58);
    patterns::glider(fb, 75, 62);
    patterns::glider(fb, 15, 80);
    patterns::glider(fb, 45, 85);
    patterns::glider(fb, 70, 80);
}
