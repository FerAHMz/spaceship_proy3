use nalgebra_glm::Vec3;
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Fragment {
    pub position: Vec3,
    pub color: Color,
    pub depth: f32,
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32) -> Self {
        Fragment {
            position: Vec3::new(x, y, 0.0),
            color,
            depth,
        }
    }
}
