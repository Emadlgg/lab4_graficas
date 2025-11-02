use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
        }
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = nalgebra_glm::normalize(&(self.center - self.eye));
        let right = nalgebra_glm::normalize(&nalgebra_glm::cross(&forward, &self.up));
        let up = nalgebra_glm::cross(&right, &forward);

        let rotated = 
            right * vector.x +
            up * vector.y +
            (-forward) * vector.z;

        rotated.normalize()
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = current_yaw + delta_yaw;
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center + Vec3::new(
            radius * new_pitch.cos() * new_yaw.cos(),
            -radius * new_pitch.sin(),
            radius * new_pitch.cos() * new_yaw.sin()
        );

        self.eye = new_eye;
        self.has_changed = true;
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = nalgebra_glm::normalize(&(self.center - self.eye));
        self.eye += direction * delta;
        self.has_changed = true;
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(&self.eye, &self.center, &self.up)
    }

    pub fn get_projection_matrix(&self, aspect: f32) -> Mat4 {
        perspective(aspect, PI / 4.0, 0.1, 1000.0)
    }
}