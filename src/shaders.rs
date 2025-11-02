use nalgebra_glm::{Vec3, Vec4, Mat4};
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::color::Color;

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: f32,
    pub light_dir: Vec3,
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let ndc_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * ndc_position;

    let model_mat3 = Mat4::new(
        uniforms.model_matrix[(0, 0)], uniforms.model_matrix[(0, 1)], uniforms.model_matrix[(0, 2)], 0.0,
        uniforms.model_matrix[(1, 0)], uniforms.model_matrix[(1, 1)], uniforms.model_matrix[(1, 2)], 0.0,
        uniforms.model_matrix[(2, 0)], uniforms.model_matrix[(2, 1)], uniforms.model_matrix[(2, 2)], 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    let normal4 = Vec4::new(vertex.normal.x, vertex.normal.y, vertex.normal.z, 0.0);
    let transformed_normal = model_mat3 * normal4;
    let final_normal = Vec3::new(transformed_normal.x, transformed_normal.y, transformed_normal.z).normalize();

    let mut new_vertex = vertex.clone();
    new_vertex.transformed_position = Vec3::new(screen_position.x, screen_position.y, screen_position.z);
    new_vertex.transformed_normal = final_normal;

    new_vertex
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: &str) -> Color {
    match shader_type {
        "static_color" => static_color_shader(fragment),
        "diffuse" => diffuse_shader(fragment, uniforms),
        "cel_shading" => cel_shading_shader(fragment, uniforms),
        "procedural" => procedural_shader(fragment, uniforms),
        "normal_map" => normal_map_shader(fragment),
        _ => fragment.color
    }
}

fn static_color_shader(_fragment: &Fragment) -> Color {
    Color::new(255, 255, 0)
}

fn diffuse_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let light_dir = uniforms.light_dir.normalize();
    let normal = fragment.normal.normalize();
    
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    let ambient = 0.3;
    
    let intensity = ambient + diffuse * 0.7;
    
    let base_color = Color::new(255, 200, 100);
    base_color * intensity
}

fn cel_shading_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let light_dir = uniforms.light_dir.normalize();
    let normal = fragment.normal.normalize();
    
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    
    let intensity = if diffuse > 0.8 {
        1.0
    } else if diffuse > 0.5 {
        0.6
    } else if diffuse > 0.2 {
        0.4
    } else {
        0.2
    };
    
    let base_color = Color::new(100, 150, 255);
    base_color * intensity
}

fn procedural_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let x = fragment.position.x;
    let y = fragment.position.y;
    let t = uniforms.time;
    
    let pattern = ((x * 0.1 + t).sin() * (y * 0.1 + t).cos() * 0.5 + 0.5).abs();
    
    Color::from_float(pattern, 1.0 - pattern, 0.5)
}

fn normal_map_shader(fragment: &Fragment) -> Color {
    let normal = fragment.normal.normalize();
    
    Color::from_float(
        (normal.x + 1.0) * 0.5,
        (normal.y + 1.0) * 0.5,
        (normal.z + 1.0) * 0.5,
    )
}

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_x, -sin_x, 0.0,
        0.0, sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0, 0.0, translation.x,
        0.0, scale, 0.0, translation.y,
        0.0, 0.0, scale, translation.z,
        0.0, 0.0, 0.0, 1.0,
    );

    transform_matrix * rotation_matrix
}