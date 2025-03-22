pub struct FastTuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl FastTuple {
    pub fn is_point(&self) -> bool {
        return self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        return !self.is_point()
    }
}