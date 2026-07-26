use crate::framebuffer::Framebuffer;
use crate::life;

// Coloca un organismo: `cells` son coordenadas relativas a un origen (ox, oy).
fn place(fb: &mut Framebuffer, ox: i32, oy: i32, cells: &[(i32, i32)]) {
    fb.set_current_color(life::ALIVE);
    for &(dx, dy) in cells {
        let x = ox + dx;
        let y = oy + dy;
        if x >= 0 && y >= 0 {
            fb.point(x as usize, y as usize);
        }
    }
}

// --- Still lifes (no cambian nunca) ---

pub fn block(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (0, 1), (1, 1)]);
}

pub fn beehive(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)],
    );
}

pub fn loaf(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)],
    );
}

pub fn boat(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)]);
}

pub fn tub(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(1, 0), (0, 1), (2, 1), (1, 2)]);
}

// --- Osciladores ---

pub fn blinker(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (2, 0)]);
}

pub fn toad(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)],
    );
}

pub fn beacon(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[
            (0, 0),
            (1, 0),
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 2),
            (2, 3),
            (3, 3),
        ],
    );
}

pub fn pulsar(fb: &mut Framebuffer, ox: i32, oy: i32) {
    let mut cells = Vec::new();
    // Los 4 brazos de 3 celdas se repiten en 4 filas y 4 columnas por simetría.
    for &row in &[0, 5, 7, 12] {
        for &col in &[2, 3, 4, 8, 9, 10] {
            cells.push((col, row));
        }
    }
    for &col in &[0, 5, 7, 12] {
        for &row in &[2, 3, 4, 8, 9, 10] {
            cells.push((col, row));
        }
    }
    place(fb, ox, oy, &cells);
}

// --- Spaceships ---

pub fn glider(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
}

pub fn lightweight_spaceship(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[
            (1, 0),
            (4, 0),
            (0, 1),
            (0, 2),
            (4, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 3),
        ],
    );
}

// Middleweight spaceship: la misma familia que la LWSS, un poco más ancha,
// con una muesca extra arriba (6x5, 11 células).
pub fn middleweight_spaceship(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[
            (2, 0),
            (1, 1),
            (5, 1),
            (0, 2),
            (0, 3),
            (5, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
        ],
    );
}

// Heavyweight spaceship: la más ancha de la familia LWSS/MWSS/HWSS (7x5, 13 células).
pub fn heavyweight_spaceship(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[
            (3, 0),
            (4, 0),
            (1, 1),
            (6, 1),
            (0, 2),
            (0, 3),
            (6, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
        ],
    );
}

// --- Otros osciladores y "methuselahs" (pocas células, evolución muy larga y caótica) ---

// Pentadecathlon: oscilador de período 15 (10x3).
pub fn pentadecathlon(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[
            (2, 0),
            (7, 0),
            (0, 1),
            (1, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (8, 1),
            (9, 1),
            (2, 2),
            (7, 2),
        ],
    );
}

// R-pentomino: solo 5 células, tarda 1103 generaciones en estabilizarse.
pub fn r_pentomino(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)]);
}

// Acorn: 7 células, tarda 5206 generaciones en estabilizarse.
pub fn acorn(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)],
    );
}

// Diehard: 7 células, desaparece por completo después de exactamente 130 generaciones.
pub fn diehard(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[(6, 0), (0, 1), (1, 1), (1, 2), (5, 2), (6, 2), (7, 2)],
    );
}

// --- Gun ---

// Gosper Glider Gun: dispara un glider nuevo cada 30 turnos, indefinidamente.
pub fn gosper_glider_gun(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[
            (24, 0),
            (22, 1),
            (24, 1),
            (12, 2),
            (13, 2),
            (20, 2),
            (21, 2),
            (34, 2),
            (35, 2),
            (11, 3),
            (15, 3),
            (20, 3),
            (21, 3),
            (34, 3),
            (35, 3),
            (0, 4),
            (1, 4),
            (10, 4),
            (16, 4),
            (20, 4),
            (21, 4),
            (0, 5),
            (1, 5),
            (10, 5),
            (14, 5),
            (16, 5),
            (17, 5),
            (22, 5),
            (24, 5),
            (10, 6),
            (16, 6),
            (24, 6),
            (11, 7),
            (15, 7),
            (12, 8),
            (13, 8),
        ],
    );
}
