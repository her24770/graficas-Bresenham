use crate::framebuffer::Framebuffer;

// Fondo morado oscuro (no negro puro), y una gradiente de celeste a morado
// según cuántos turnos lleva viva una celda: recién nacida = celeste,
// mientras más sobrevive, más se oscurece hacia morado. Es solo estética:
// la regla de "¿está viva?" sigue siendo binaria (get_color != BACKGROUND).
pub const BACKGROUND: u32 = 0x120024;

const PALETTE: [u32; 8] = [
    0xE0F7FA, // recién nacida
    0x80DEEA,
    0x26C6DA,
    0x29B6F6,
    0x5C6BC0,
    0x7E57C2,
    0x8E24AA,
    0x6A1B9A, // muy longeva (still life estable)
];

pub fn color_for_age(age: u32) -> u32 {
    let idx = (age as usize).min(PALETTE.len() - 1);
    PALETTE[idx]
}

// Calcula un turno completo de Conway: lee el estado de `current` (con
// get_color) y escribe el siguiente estado en `next` (con point). Los
// bordes son toroidales: el vecino más allá del borde derecho es la
// primera columna, y así con los 4 lados.
//
// `current_age`/`next_age` llevan, para cada celda, cuántos turnos seguidos
// lleva viva — solo se usan para elegir el color, no afectan las reglas.
pub fn step(current: &Framebuffer, next: &mut Framebuffer, current_age: &[u32], next_age: &mut [u32]) {
    let width = current.width as i32;
    let height = current.height as i32;

    for y in 0..height {
        for x in 0..width {
            let mut alive_neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = (x + dx).rem_euclid(width) as usize;
                    let ny = (y + dy).rem_euclid(height) as usize;
                    if current.get_color(nx, ny) != BACKGROUND {
                        alive_neighbors += 1;
                    }
                }
            }

            let idx = y as usize * current.width + x as usize;
            let is_alive = current.get_color(x as usize, y as usize) != BACKGROUND;
            let will_live = matches!((is_alive, alive_neighbors), (true, 2) | (true, 3) | (false, 3));

            if will_live {
                let age = if is_alive { current_age[idx] + 1 } else { 0 };
                next_age[idx] = age;
                next.set_current_color(color_for_age(age));
            } else {
                next_age[idx] = 0;
                next.set_current_color(BACKGROUND);
            }
            next.point(x as usize, y as usize);
        }
    }
}
