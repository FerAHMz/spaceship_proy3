use nalgebra_glm::{Vec3, Mat4};
use crate::vertex::Vertex;

pub struct Uniforms {
    pub model_matrix: Mat4,
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec3::new(vertex.position.x, vertex.position.y, vertex.position.z);
    
    // Transform position
    let transformed_position = uniforms.model_matrix * Vec3::new(position.x, position.y, position.z).push(1.0);
    let transformed_pos = Vec3::new(transformed_position.x, transformed_position.y, transformed_position.z);
    
    // Transform normal (assuming no non-uniform scaling)
    let transformed_normal = uniforms.model_matrix * vertex.normal.push(0.0);
    let transformed_norm = Vec3::new(transformed_normal.x, transformed_normal.y, transformed_normal.z);

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: transformed_pos,
        transformed_normal: transformed_norm,
    }
}
