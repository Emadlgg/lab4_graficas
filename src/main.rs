mod color;
mod framebuffer;
mod triangle;
mod obj_loader;

use crate::color::Color;
use crate::framebuffer::{Framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::triangle::Triangle;
use crate::obj_loader::Model;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, Mat4};

fn create_rotation_matrix(rotation: &Vec3) -> Mat4 {
    let rotation_x = nalgebra_glm::rotation(rotation.x, &Vec3::new(1.0, 0.0, 0.0));
    let rotation_y = nalgebra_glm::rotation(rotation.y, &Vec3::new(0.0, 1.0, 0.0));
    let rotation_z = nalgebra_glm::rotation(rotation.z, &Vec3::new(0.0, 0.0, 1.0));
    rotation_z * rotation_y * rotation_x
}

fn apply_transform(matrix: &Mat4, point: &Vec3) -> Vec3 {
    let x = matrix[(0, 0)] * point.x + matrix[(0, 1)] * point.y + matrix[(0, 2)] * point.z + matrix[(0, 3)];
    let y = matrix[(1, 0)] * point.x + matrix[(1, 1)] * point.y + matrix[(1, 2)] * point.z + matrix[(1, 3)];
    let z = matrix[(2, 0)] * point.x + matrix[(2, 1)] * point.y + matrix[(2, 2)] * point.z + matrix[(2, 3)];
    Vec3::new(x, y, z)
}

fn calculate_normal(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> Vec3 {
    let edge1 = v2 - v1;
    let edge2 = v3 - v1;
    nalgebra_glm::normalize(&nalgebra_glm::cross(&edge1, &edge2))
}

fn render(framebuffer: &mut Framebuffer, model: &Model, rotation: &Vec3, use_lighting: bool) {
    let rotation_matrix = create_rotation_matrix(rotation);
    
    // Dirección de la luz (desde arriba-derecha-frente)
    let light_dir = nalgebra_glm::normalize(&Vec3::new(0.5, -0.5, -1.0));
    
    // Recorrer todas las caras del modelo
    for face in &model.faces {
        if face.len() >= 3 {
            let idx1 = face[0];
            let idx2 = face[1];
            let idx3 = face[2];

            if idx1 < model.vertices.len() && 
               idx2 < model.vertices.len() && 
               idx3 < model.vertices.len() {
                
                // Obtener vértices originales centrados
                let center = Vec3::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0, 0.0);
                let v1_centered = model.vertices[idx1];
                let v2_centered = model.vertices[idx2];
                let v3_centered = model.vertices[idx3];

                // Aplicar rotación
                let v1_rot = apply_transform(&rotation_matrix, &v1_centered);
                let v2_rot = apply_transform(&rotation_matrix, &v2_centered);
                let v3_rot = apply_transform(&rotation_matrix, &v3_centered);

                let v1 = v1_rot + center;
                let v2 = v2_rot + center;
                let v3 = v3_rot + center;

                // Calcular iluminación si está activada
                if use_lighting {
                    let normal = calculate_normal(&v1_rot, &v2_rot, &v3_rot);
                    let intensity = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
                    
                    // Color base amarillo con iluminación
                    let base_color = Color::new(255, 255, 0);
                    let lit_color = Color::new(
                        (base_color.r as f32 * (0.3 + 0.7 * intensity)) as u8,
                        (base_color.g as f32 * (0.3 + 0.7 * intensity)) as u8,
                        (base_color.b as f32 * (0.3 + 0.7 * intensity)) as u8,
                    );
                    framebuffer.set_current_color(lit_color);
                }

                // Dibujar el triángulo
                let triangle = Triangle::new(v1, v2, v3);
                triangle.draw(framebuffer);
            }
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Software Renderer - Spaceship [WASD/Flechas: Rotar | L: Luz | R: Reset]",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("No se pudo crear la ventana: {}", e);
    });

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    framebuffer.set_background_color(Color::new(0, 0, 0));

    let mut model = Model::load_from_file("spaceship.obj")
        .expect("No se pudo cargar el archivo OBJ");

    model.normalize_and_center(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32, 300.0);

    println!("Modelo cargado:");
    println!("  Vértices: {}", model.vertices.len());
    println!("  Caras: {}", model.faces.len());
    println!("\nControles:");
    println!("  W/S o ↑/↓: Rotar en X");
    println!("  A/D o ←/→: Rotar en Y");
    println!("  Q/E: Rotar en Z");
    println!("  L: Activar/desactivar iluminación");
    println!("  R: Resetear rotación");
    println!("  ESC: Salir");

    // Variables para rotación
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let rotation_speed = 0.05;
    let mut use_lighting = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Controles de rotación
        if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
            rotation.x += rotation_speed;
        }
        if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
            rotation.x -= rotation_speed;
        }
        if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
            rotation.y += rotation_speed;
        }
        if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
            rotation.y -= rotation_speed;
        }
        if window.is_key_down(Key::Q) {
            rotation.z += rotation_speed;
        }
        if window.is_key_down(Key::E) {
            rotation.z -= rotation_speed;
        }
        
        // Toggle iluminación
        if window.is_key_pressed(Key::L, minifb::KeyRepeat::No) {
            use_lighting = !use_lighting;
            println!("Iluminación: {}", if use_lighting { "Activada" } else { "Desactivada" });
        }
        
        // Reset rotación
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            rotation = Vec3::new(0.0, 0.0, 0.0);
            println!("Rotación reseteada");
        }

        framebuffer.clear();

        if !use_lighting {
            framebuffer.set_current_color(Color::new(255, 255, 0));
        }

        render(&mut framebuffer, &model, &rotation, use_lighting);

        window
            .update_with_buffer(&framebuffer.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}