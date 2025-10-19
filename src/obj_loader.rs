use nalgebra_glm::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Vec<usize>>,
}

impl Model {
    pub fn load_from_file(filename: &str) -> Result<Self, String> {
        let file = File::open(filename)
            .map_err(|e| format!("Error abriendo archivo: {}", e))?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error leyendo línea: {}", e))?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            
            match parts.get(0) {
                Some(&"v") => {
                    // Vertex
                    if parts.len() >= 4 {
                        let x: f32 = parts[1].parse().unwrap_or(0.0);
                        let y: f32 = parts[2].parse().unwrap_or(0.0);
                        let z: f32 = parts[3].parse().unwrap_or(0.0);
                        vertices.push(Vec3::new(x, y, z));
                    }
                }
                Some(&"f") => {
                    // Face
                    let mut face_indices = Vec::new();
                    for i in 1..parts.len() {
                        // Los índices pueden venir como "v", "v/vt", o "v/vt/vn"
                        let index_str = parts[i].split('/').next().unwrap_or("0");
                        let index: usize = index_str.parse().unwrap_or(1);
                        // OBJ usa índices base 1, convertir a base 0
                        face_indices.push(index - 1);
                    }
                    
                    // Si la cara tiene más de 3 vértices, triangular
                    if face_indices.len() >= 3 {
                        // Triangulación simple en abanico
                        for i in 1..(face_indices.len() - 1) {
                            faces.push(vec![face_indices[0], face_indices[i], face_indices[i + 1]]);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Model { vertices, faces })
    }

    pub fn get_bounds(&self) -> (Vec3, Vec3) {
        if self.vertices.is_empty() {
            return (Vec3::zeros(), Vec3::zeros());
        }

        let mut min = self.vertices[0];
        let mut max = self.vertices[0];

        for v in &self.vertices {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);
            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        (min, max)
    }

    pub fn normalize_and_center(&mut self, _screen_width: f32, _screen_height: f32, scale: f32) {
        let (min, max) = self.get_bounds();
        let center = (min + max) / 2.0;
        let size = max - min;
        let max_dimension = size.x.max(size.y).max(size.z);

        if max_dimension == 0.0 {
            return;
        }

        let scale_factor = scale / max_dimension;

        for v in &mut self.vertices {
            // Centrar en el origen
            *v = *v - center;
            
            // Escalar
            *v = *v * scale_factor;
        }
    }
}