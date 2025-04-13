#[path = "./color.rs"]
mod color;
use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,

    // Note 2D array is inefficient, but logically most simple.
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        // Refer to pixel as pixel[width, height]
        let pixels: Vec<Vec<Color>> = vec![vec![Color::new(0.0, 0.0, 0.0); height]; width];
        Canvas {
            width,
            height,
            pixels,
        }
    }
}
