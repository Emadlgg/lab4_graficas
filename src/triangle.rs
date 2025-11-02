use crate::framebuffer::Framebuffer;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use nalgebra_glm::Vec3;

pub struct Triangle {
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
}

impl Triangle {
    pub fn new_from_vertices(v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
        Triangle { v1, v2, v3 }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) -> Vec<Fragment> {
        let mut fragments = Vec::new();

        let v1_pos = self.v1.transformed_position;
        let v2_pos = self.v2.transformed_position;
        let v3_pos = self.v3.transformed_position;

        let (x1, y1, z1) = (v1_pos.x as i32, v1_pos.y as i32, v1_pos.z);
        let (x2, y2, z2) = (v2_pos.x as i32, v2_pos.y as i32, v2_pos.z);
        let (x3, y3, z3) = (v3_pos.x as i32, v3_pos.y as i32, v3_pos.z);

        let min_x = x1.min(x2).min(x3).max(0);
        let max_x = x1.max(x2).max(x3).min(framebuffer.width as i32 - 1);
        let min_y = y1.min(y2).min(y3).max(0);
        let max_y = y1.max(y2).max(y3).min(framebuffer.height as i32 - 1);

        let edge1 = v2_pos - v1_pos;
        let edge2 = v3_pos - v1_pos;
        let triangle_normal = nalgebra_glm::cross(&edge1, &edge2).normalize();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let (w1, w2, w3) = barycentric_coordinates(
                    x as f32, y as f32,
                    x1 as f32, y1 as f32,
                    x2 as f32, y2 as f32,
                    x3 as f32, y3 as f32,
                );

                if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                    let depth = z1 * w1 + z2 * w2 + z3 * w3;
                    
                    let normal = (self.v1.transformed_normal * w1 + 
                                 self.v2.transformed_normal * w2 + 
                                 self.v3.transformed_normal * w3).normalize();

                    let intensity = 1.0;

                    let fragment = Fragment::new_with_normal(
                        x as f32,
                        y as f32,
                        framebuffer.current_color,
                        depth,
                        normal,
                        intensity,
                    );

                    fragments.push(fragment);
                }
            }
        }

        fragments
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