mod color;
mod framebuffer;
mod triangle;
mod obj_loader;
mod vertex;
mod fragment;
mod shaders;
mod camera;

use crate::color::Color;
use crate::framebuffer::{Framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::triangle::Triangle;
use crate::obj_loader::Model;
use crate::vertex::Vertex;
use crate::shaders::{vertex_shader, fragment_shader, create_model_matrix, create_viewport_matrix, Uniforms};
use crate::camera::Camera;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, Mat4};
use std::time::Instant;

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader_type: &str,
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push(Triangle::new_from_vertices(
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ));
        }
    }

    let mut all_fragments = Vec::new();
    for triangle in &triangles {
        let fragments = triangle.draw(framebuffer);
        all_fragments.extend(fragments);
    }

    for fragment in all_fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        
        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, uniforms, shader_type);
            framebuffer.set_current_color(shaded_color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let mut window = Window::new(
        "3D Renderer [WASD/Flechas: Cámara | 1-5: Shaders | R: Reset | ESC: Salir]",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("No se pudo crear la ventana: {}", e);
    });

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    framebuffer.set_background_color(Color::new(20, 20, 40));

    let mut model = Model::load_from_file("spaceship.obj")
        .expect("No se pudo cargar el archivo OBJ");
    model.normalize_and_center(1.5);

    println!("Modelo cargado:");
    println!("  Vértices: {}", model.vertices.len());
    println!("\nControles:");
    println!("  W/S o ↑/↓: Orbitar verticalmente");
    println!("  A/D o ←/→: Orbitar horizontalmente");
    println!("  Q/E: Zoom");
    println!("  1: Shader estático");
    println!("  2: Shader difuso");
    println!("  3: Cel Shading");
    println!("  4: Shader procedural");
    println!("  5: Normal Map");
    println!("  R: Resetear cámara");
    println!("  ESC: Salir");

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut current_shader = "diffuse";
    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = start_time.elapsed().as_secs_f32();

        // Controles de cámara
        if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
            camera.orbit(0.0, 0.05);
        }
        if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
            camera.orbit(0.0, -0.05);
        }
        if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
            camera.orbit(-0.05, 0.0);
        }
        if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
            camera.orbit(0.05, 0.0);
        }
        if window.is_key_down(Key::Q) {
            camera.zoom(-0.1);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(0.1);
        }

        // Selección de shaders
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            current_shader = "static_color";
            println!("Shader: Estático");
        }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
            current_shader = "diffuse";
            println!("Shader: Difuso");
        }
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
            current_shader = "cel_shading";
            println!("Shader: Cel Shading");
        }
        if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) {
            current_shader = "procedural";
            println!("Shader: Procedural");
        }
        if window.is_key_pressed(Key::Key5, minifb::KeyRepeat::No) {
            current_shader = "normal_map";
            println!("Shader: Normal Map");
        }

        // Reset cámara
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            camera = Camera::new(
                Vec3::new(0.0, 0.0, 5.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            );
            rotation = Vec3::new(0.0, 0.0, 0.0);
            println!("Cámara reseteada");
        }

        framebuffer.clear();

        let model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.0, rotation);
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = camera.get_projection_matrix(SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32);
        let viewport_matrix = create_viewport_matrix(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            light_dir: Vec3::new(0.5, -0.5, -1.0),
        };

        render(&mut framebuffer, &uniforms, &model.vertices, current_shader);

        window
            .update_with_buffer(&framebuffer.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}