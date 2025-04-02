pub struct FastTuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl FastTuple {
    // Note that this uses == for this, but 1.0 and 0.0 should never result in loss of precision.
    pub fn is_point(&self) -> bool {
        return self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        return !self.is_point()
    }

    pub fn point(x: f32, y: f32, z: f32) -> FastTuple {
        return FastTuple {x: x, y: y, z: z, w: 1.0}
    }

    pub fn vector(x: f32, y: f32, z: f32) -> FastTuple {
        return FastTuple {x: x, y: y, z: z, w: 0.0}
    }
}

impl PartialEq for FastTuple {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 0.00001;
        let x_eq = (self.x - other.x).abs() < epsilon;
        let y_eq = (self.y - other.y).abs() < epsilon;
        let z_eq = (self.z - other.z).abs() < epsilon;
        return x_eq && y_eq && z_eq && self.w == other.w;
    }
}