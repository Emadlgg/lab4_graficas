use nalgebra_glm::Vec2;
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub normal: nalgebra_glm::Vec3,
    pub intensity: f32,
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
            normal: nalgebra_glm::Vec3::new(0.0, 0.0, 1.0),
            intensity: 1.0,
        }
    }

    pub fn new_with_normal(x: f32, y: f32, color: Color, depth: f32, normal: nalgebra_glm::Vec3, intensity: f32) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
            normal,
            intensity,
        }
    }
}