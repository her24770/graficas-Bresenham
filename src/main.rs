mod framebuffer;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use polygon::{draw_polygon_outline, fill_polygon};

const CANVAS_WIDTH: usize = 800;
const CANVAS_HEIGHT: usize = 450;

fn main() {
    let mut fb = Framebuffer::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    fb.clear((30, 30, 30));

    let poly1: [(i32, i32); 10] = [
        (165, 380),
        (185, 360),
        (180, 330),
        (207, 345),
        (233, 330),
        (230, 360),
        (250, 380),
        (220, 385),
        (205, 410),
        (193, 383),
    ];

    let poly2: [(i32, i32); 4] = [(321, 335), (288, 286), (339, 251), (374, 302)];

    let poly3: [(i32, i32); 3] = [(377, 249), (411, 197), (436, 249)];

    let poly4: [(i32, i32); 18] = [
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180),
    ];

    // Agujero dentro del polígono 4: no debe quedar pintado.
    let poly5: [(i32, i32); 4] = [(682, 175), (708, 120), (735, 148), (739, 170)];

    fill_polygon(&mut fb, &[&poly1], (255, 215, 0)); // dorado
    draw_polygon_outline(&mut fb, &poly1, (255, 255, 255));

    fill_polygon(&mut fb, &[&poly2], (255, 165, 0)); // naranja
    draw_polygon_outline(&mut fb, &poly2, (255, 255, 255));

    fill_polygon(&mut fb, &[&poly3], (0, 255, 255)); // aqua
    draw_polygon_outline(&mut fb, &poly3, (255, 255, 255));

    // El polígono 4 y su agujero (polígono 5) se rellenan juntos: la regla
    // par-impar deja sin pintar la zona donde ambos contornos se solapan.
    fill_polygon(&mut fb, &[&poly4, &poly5], (200, 162, 200)); // lila
    draw_polygon_outline(&mut fb, &poly4, (255, 255, 255));
    draw_polygon_outline(&mut fb, &poly5, (255, 255, 255));

    save_png(&fb, "out.png");
    println!("Imagen generada: out.png");
}

fn save_png(fb: &Framebuffer, path: &str) {
    let img = image::RgbImage::from_raw(fb.width as u32, fb.height as u32, fb.data.clone())
        .expect("el tamaño del buffer no coincide con width * height * 3");
    img.save(path).expect("no se pudo guardar el PNG");
}
