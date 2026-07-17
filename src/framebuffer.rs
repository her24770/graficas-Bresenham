// Framebuffer: buffer de píxeles en memoria, formato RGB (3 bytes por píxel).

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>, // RGB plano: data[(y * width + x) * 3 + canal]
    pub current_color: (u8, u8, u8),
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            data: vec![0; width * height * 3],
            current_color: (255, 255, 255),
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        for pixel in self.data.chunks_mut(3) {
            pixel[0] = color.0;
            pixel[1] = color.1;
            pixel[2] = color.2;
        }
    }

    pub fn set_current_color(&mut self, color: (u8, u8, u8)) {
        self.current_color = color;
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }
        let idx = (y as usize * self.width + x as usize) * 3;
        self.data[idx] = self.current_color.0;
        self.data[idx + 1] = self.current_color.1;
        self.data[idx + 2] = self.current_color.2;
    }
}
