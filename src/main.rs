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

// Un organismo a colocar: la función que lo dibuja, y el tamaño de su
// caja (para poder revisar que no se encime con otros ya colocados).
struct Organism {
    place: fn(&mut Framebuffer, i32, i32),
    w: i32,
    h: i32,
}

// Generador pseudo-aleatorio mínimo (xorshift32), con semilla fija para que
// el patrón inicial sea reproducible entre corridas.
struct Rng(u32);

impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}

// Dispersa organismos por todo el tablero en posiciones al azar, revisando
// que las cajas de cada organismo no se encimen con las ya colocadas.
fn place_initial_pattern(fb: &mut Framebuffer) {
    let organisms = [
        Organism { place: patterns::gosper_glider_gun, w: 36, h: 9 },
        Organism { place: patterns::pulsar, w: 13, h: 13 },
        Organism { place: patterns::heavyweight_spaceship, w: 7, h: 5 },
        Organism { place: patterns::middleweight_spaceship, w: 6, h: 5 },
        Organism { place: patterns::pentadecathlon, w: 10, h: 3 },
        Organism { place: patterns::lightweight_spaceship, w: 5, h: 4 },
        Organism { place: patterns::diehard, w: 8, h: 3 },
        Organism { place: patterns::acorn, w: 7, h: 3 },
        Organism { place: patterns::r_pentomino, w: 3, h: 3 },
        Organism { place: patterns::loaf, w: 4, h: 4 },
        Organism { place: patterns::beehive, w: 4, h: 3 },
        Organism { place: patterns::boat, w: 3, h: 3 },
        Organism { place: patterns::tub, w: 3, h: 3 },
        Organism { place: patterns::beacon, w: 4, h: 4 },
        Organism { place: patterns::toad, w: 4, h: 2 },
        Organism { place: patterns::block, w: 2, h: 2 },
        Organism { place: patterns::blinker, w: 3, h: 1 },
        Organism { place: patterns::glider, w: 3, h: 3 },
    ];

    // Cuántas veces se repite cada organismo (los grandes una sola vez,
    // los chicos varias veces para llenar bien la pantalla).
    let repeats = [1, 1, 1, 1, 1, 3, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 6];

    let mut rng = Rng(0x9E3779B9);
    let mut placed: Vec<(i32, i32, i32, i32)> = Vec::new();
    let margin = 2;

    for (organism, &count) in organisms.iter().zip(repeats.iter()) {
        for _ in 0..count {
            for _attempt in 0..300 {
                let max_x = FRAMEBUFFER_WIDTH as i32 - organism.w - 4;
                let max_y = FRAMEBUFFER_HEIGHT as i32 - organism.h - 4;
                let x = 2 + (rng.next() % max_x as u32) as i32;
                let y = 2 + (rng.next() % max_y as u32) as i32;

                let overlaps = placed.iter().any(|&(px, py, pw, ph)| {
                    x - margin < px + pw + margin
                        && x + organism.w + margin > px - margin
                        && y - margin < py + ph + margin
                        && y + organism.h + margin > py - margin
                });

                if !overlaps {
                    (organism.place)(fb, x, y);
                    placed.push((x, y, organism.w, organism.h));
                    break;
                }
            }
        }
    }
}
