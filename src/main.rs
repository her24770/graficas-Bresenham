mod bmp;
mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::draw_line;

fn main() -> std::io::Result<()> {
    let width = 800;
    let height = 600;
    let mut fb = Framebuffer::new(width, height);
    fb.clear((0, 0, 0)); // fondo negro
    fb.set_current_color((255, 255, 255));

    // Polígono simple (pentágono) conectando sus vértices.
    let vertices = [
        (400, 100),
        (600, 250),
        (520, 480),
        (280, 480),
        (200, 250),
    ];

    for i in 0..vertices.len() {
        let (x0, y0) = vertices[i];
        let (x1, y1) = vertices[(i + 1) % vertices.len()];
        draw_line(&mut fb, x0, y0, x1, y1);
    }

    bmp::save_bmp(&fb, "out.bmp")?;
    println!("Imagen generada: out.bmp");

    Ok(())
}
