use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Model {
    pub vertices: Vec<Vertex>,
}

impl Model {
    pub fn load_from_file(filename: &str) -> Result<Self, String> {
        let file = File::open(filename)
            .map_err(|e| format!("Error abriendo archivo: {}", e))?;
        let reader = BufReader::new(file);

        let mut temp_positions = Vec::new();
        let mut temp_normals = Vec::new();
        let mut temp_texcoords = Vec::new();
        let mut vertices = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error leyendo línea: {}", e))?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.get(0) {
                Some(&"v") => {
                    if parts.len() >= 4 {
                        let x: f32 = parts[1].parse().unwrap_or(0.0);
                        let y: f32 = parts[2].parse().unwrap_or(0.0);
                        let z: f32 = parts[3].parse().unwrap_or(0.0);
                        temp_positions.push(Vec3::new(x, y, z));
                    }
                }
                Some(&"vn") => {
                    if parts.len() >= 4 {
                        let x: f32 = parts[1].parse().unwrap_or(0.0);
                        let y: f32 = parts[2].parse().unwrap_or(0.0);
                        let z: f32 = parts[3].parse().unwrap_or(0.0);
                        temp_normals.push(Vec3::new(x, y, z));
                    }
                }
                Some(&"vt") => {
                    if parts.len() >= 3 {
                        let u: f32 = parts[1].parse().unwrap_or(0.0);
                        let v: f32 = parts[2].parse().unwrap_or(0.0);
                        temp_texcoords.push(Vec2::new(u, v));
                    }
                }
                Some(&"f") => {
                    let mut face_vertices = Vec::new();

                    for i in 1..parts.len() {
                        let indices: Vec<&str> = parts[i].split('/').collect();

                        let pos_idx: usize = indices[0].parse::<usize>().unwrap_or(1) - 1;
                        let tex_idx: usize = if indices.len() > 1 && !indices[1].is_empty() {
                            indices[1].parse::<usize>().unwrap_or(1) - 1
                        } else {
                            0
                        };
                        let norm_idx: usize = if indices.len() > 2 {
                            indices[2].parse::<usize>().unwrap_or(1) - 1
                        } else {
                            0
                        };

                        let position = temp_positions.get(pos_idx).cloned().unwrap_or(Vec3::zeros());
                        let normal = temp_normals.get(norm_idx).cloned().unwrap_or(Vec3::new(0.0, 1.0, 0.0));
                        let tex_coords = temp_texcoords.get(tex_idx).cloned().unwrap_or(Vec2::zeros());

                        face_vertices.push(Vertex::new(position, normal, tex_coords));
                    }

                    // Triangulación en abanico
                    for i in 1..(face_vertices.len() - 1) {
                        vertices.push(face_vertices[0].clone());
                        vertices.push(face_vertices[i].clone());
                        vertices.push(face_vertices[i + 1].clone());
                    }
                }
                _ => {}
            }
        }

        Ok(Model { vertices })
    }

    pub fn get_bounds(&self) -> (Vec3, Vec3) {
        if self.vertices.is_empty() {
            return (Vec3::zeros(), Vec3::zeros());
        }

        let mut min = self.vertices[0].position;
        let mut max = self.vertices[0].position;

        for vertex in &self.vertices {
            let v = vertex.position;
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);
            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        (min, max)
    }

    pub fn normalize_and_center(&mut self, scale: f32) {
        let (min, max) = self.get_bounds();
        let center = (min + max) / 2.0;
        let size = max - min;
        let max_dimension = size.x.max(size.y).max(size.z);

        if max_dimension == 0.0 {
            return;
        }

        let scale_factor = scale / max_dimension;

        for vertex in &mut self.vertices {
            vertex.position = (vertex.position - center) * scale_factor;
        }
    }
}