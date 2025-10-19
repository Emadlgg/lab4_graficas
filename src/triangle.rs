use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Triangle { v1, v2, v3 }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        // Convertir las coordenadas a coordenadas de pantalla
        let (x1, y1) = (self.v1.x as i32, self.v1.y as i32);
        let (x2, y2) = (self.v2.x as i32, self.v2.y as i32);
        let (x3, y3) = (self.v3.x as i32, self.v3.y as i32);

        // Encontrar el bounding box del triángulo
        let min_x = x1.min(x2).min(x3).max(0);
        let max_x = x1.max(x2).max(x3).min(framebuffer.width as i32 - 1);
        let min_y = y1.min(y2).min(y3).max(0);
        let max_y = y1.max(y2).max(y3).min(framebuffer.height as i32 - 1);

        // Iterar sobre cada pixel en el bounding box
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                // Calcular coordenadas baricéntricas
                let (w1, w2, w3) = barycentric_coordinates(
                    x as f32, y as f32,
                    x1 as f32, y1 as f32,
                    x2 as f32, y2 as f32,
                    x3 as f32, y3 as f32,
                );

                // Si el punto está dentro del triángulo, dibujarlo
                if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                    framebuffer.point(x as usize, y as usize);
                }
            }
        }
    }
}

fn barycentric_coordinates(
    px: f32, py: f32,
    ax: f32, ay: f32,
    bx: f32, by: f32,
    cx: f32, cy: f32,
) -> (f32, f32, f32) {
    let denom = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy);
    
    if denom.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    let w1 = ((by - cy) * (px - cx) + (cx - bx) * (py - cy)) / denom;
    let w2 = ((cy - ay) * (px - cx) + (ax - cx) * (py - cy)) / denom;
    let w3 = 1.0 - w1 - w2;

    (w1, w2, w3)
}