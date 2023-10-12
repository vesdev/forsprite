use nalgebra::Vector2;

pub struct Rect {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            min: Vector2::new(x, y),
            max: Vector2::new(x + w, y + h),
        }
    }
}
