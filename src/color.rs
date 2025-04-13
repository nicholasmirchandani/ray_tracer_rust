pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 0.00001;
        let red_eq = (self.red - other.red).abs() < epsilon;
        let green_eq = (self.green - other.green).abs() < epsilon;
        let blue_eq = (self.blue - other.blue).abs() < epsilon;
        return red_eq && green_eq && blue_eq;
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl std::marker::Copy for Color {}

impl std::clone::Clone for Color {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}
