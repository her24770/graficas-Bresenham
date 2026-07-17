use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, Write};

// Escritor de BMP sin dependencias externas: BITMAPFILEHEADER + BITMAPINFOHEADER
// + datos de píxeles de 24 bpp (BGR, filas de abajo hacia arriba, con padding a 4 bytes).
pub fn save_bmp(fb: &Framebuffer, path: &str) -> io::Result<()> {
    let width = fb.width as u32;
    let height = fb.height as u32;

    let row_size = (width * 3 + 3) & !3; // padding a múltiplo de 4
    let pixel_data_size = row_size * height;
    let file_size = 14 + 40 + pixel_data_size;

    let mut file = File::create(path)?;

    // --- BITMAPFILEHEADER (14 bytes) ---
    file.write_all(b"BM")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?; // reservado
    file.write_all(&(14u32 + 40u32).to_le_bytes())?; // offset a datos de píxeles

    // --- BITMAPINFOHEADER (40 bytes) ---
    file.write_all(&40u32.to_le_bytes())?; // tamaño del header
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(height as i32).to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?; // planos
    file.write_all(&24u16.to_le_bytes())?; // bits por píxel
    file.write_all(&0u32.to_le_bytes())?; // sin compresión
    file.write_all(&pixel_data_size.to_le_bytes())?;
    file.write_all(&0i32.to_le_bytes())?; // resolución X
    file.write_all(&0i32.to_le_bytes())?; // resolución Y
    file.write_all(&0u32.to_le_bytes())?; // colores en la paleta
    file.write_all(&0u32.to_le_bytes())?; // colores importantes

    // --- Datos de píxeles (BMP se guarda de abajo hacia arriba) ---
    let padding = vec![0u8; (row_size - width * 3) as usize];
    for y in (0..fb.height).rev() {
        for x in 0..fb.width {
            let idx = (y * fb.width + x) * 3;
            let r = fb.data[idx];
            let g = fb.data[idx + 1];
            let b = fb.data[idx + 2];
            file.write_all(&[b, g, r])?; // BMP usa orden BGR
        }
        file.write_all(&padding)?;
    }

    Ok(())
}
