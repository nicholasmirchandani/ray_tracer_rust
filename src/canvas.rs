pub struct Canvas {
    pub width : u32;
    pub height : u32;
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Canvas { width, height }
    }
}