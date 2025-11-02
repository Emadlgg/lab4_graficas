use crate::color::Color;

pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 600;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            background_color: Color::black(),
            current_color: Color::new(255, 255, 255),
        }
    }

    pub fn clear(&mut self) {
        let bg_color = self.background_color.to_hex();
        for pixel in self.buffer.iter_mut() {
            *pixel = bg_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            
            if depth < self.zbuffer[index] {
                self.buffer[index] = self.current_color.to_hex();
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
}