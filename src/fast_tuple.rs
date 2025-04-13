pub struct FastTuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl FastTuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> FastTuple {
        FastTuple { x, y, z, w }
    }

    // Note that this uses == for this, but 1.0 and 0.0 should never result in loss of precision.
    pub fn is_point(&self) -> bool {
        return self.w == 1.0;
    }

    pub fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }

    pub fn magnitude(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
    }

    pub fn point(x: f32, y: f32, z: f32) -> FastTuple {
        return FastTuple {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        };
    }

    pub fn vector(x: f32, y: f32, z: f32) -> FastTuple {
        return FastTuple {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        };
    }

    pub fn normalize(ft: FastTuple) -> FastTuple {
        let mag = ft.magnitude();
        if mag == 0.0 {
            return FastTuple {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            };
        }

        FastTuple {
            x: ft.x / mag,
            y: ft.y / mag,
            z: ft.z / mag,
            // Normalizing a vector => w  is always 0; potential optimization.
            w: ft.w / mag,
        }
    }

    pub fn dot(a: FastTuple, b: FastTuple) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    pub fn cross(a: FastTuple, b: FastTuple) -> FastTuple {
        FastTuple {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
            // Vector!
            w: 0.0,
        }
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

impl std::ops::Add for FastTuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::Sub for FastTuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::fmt::Display for FastTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl std::marker::Copy for FastTuple {}

impl std::clone::Clone for FastTuple {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::ops::Neg for FastTuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f32> for FastTuple {
    type Output = Self;

    // Scalar multiplication
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl std::ops::Div<f32> for FastTuple {
    type Output = Self;

    // Scalar division
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}
