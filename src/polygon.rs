use crate::framebuffer::Framebuffer;
use crate::line::draw_line;

// Relleno de polígonos con Scanline Fill y regla par-impar (tecnica para el vacio)
pub fn fill_polygon(fb: &mut Framebuffer, contours: &[&[(i32, i32)]], color: (u8, u8, u8)) {
    let mut edges: Vec<((f64, f64), (f64, f64))> = Vec::new();
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for contour in contours {
        let n = contour.len();
        for i in 0..n {
            let (x1, y1) = contour[i];
            let (x2, y2) = contour[(i + 1) % n];
            min_y = min_y.min(y1).min(y2);
            max_y = max_y.max(y1).max(y2);
            if y1 != y2 {
                edges.push(((x1 as f64, y1 as f64), (x2 as f64, y2 as f64)));
            }
        }
    }

    fb.set_current_color(color);

    for y in min_y..=max_y {
        // se muestrea en el centro del píxel
        let yf = y as f64 + 0.5;

        let mut xs: Vec<f64> = Vec::new();
        for &((x1, y1), (x2, y2)) in &edges {
            let (ymin, ymax) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            if yf >= ymin && yf < ymax {
                let x = x1 + (yf - y1) / (y2 - y1) * (x2 - x1);
                xs.push(x);
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut i = 0;
        while i + 1 < xs.len() {
            // se redondea hacia adentro (ceil/floor) para no pintar fuera del borde del polígono
            let x_start = xs[i].ceil() as i32;
            let x_end = xs[i + 1].floor() as i32;
            for x in x_start..=x_end {
                fb.set_pixel(x, y);
            }
            i += 2;
        }
    }
}

// Dibuja el contorno de un polígono cerrado uniendo cada vértice con el siguiente.
pub fn draw_polygon_outline(fb: &mut Framebuffer, vertices: &[(i32, i32)], color: (u8, u8, u8)) {
    fb.set_current_color(color);
    let n = vertices.len();
    for i in 0..n {
        let (x0, y0) = vertices[i];
        let (x1, y1) = vertices[(i + 1) % n];
        draw_line(fb, x0, y0, x1, y1);
    }
}
