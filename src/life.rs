use crate::framebuffer::Framebuffer;

pub const ALIVE: u32 = 0xFFFFFF;
pub const DEAD: u32 = 0x000000;

// Calcula un turno completo de Conway: lee el estado de `current` (con
// get_color) y escribe el siguiente estado en `next` (con point). Los
// bordes son toroidales: el vecino más allá del borde derecho es la
// primera columna, y así con los 4 lados.
pub fn step(current: &Framebuffer, next: &mut Framebuffer) {
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
                    if current.get_color(nx, ny) == ALIVE {
                        alive_neighbors += 1;
                    }
                }
            }

            let is_alive = current.get_color(x as usize, y as usize) == ALIVE;
            let will_live = matches!((is_alive, alive_neighbors), (true, 2) | (true, 3) | (false, 3));

            next.set_current_color(if will_live { ALIVE } else { DEAD });
            next.point(x as usize, y as usize);
        }
    }
}
