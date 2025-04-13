#[path = "../src/canvas.rs"]
mod canvas;
#[path = "../src/color.rs"]
mod color;
mod test_hack;

#[cfg(test)]
mod canvas_test {
    // Import the necessary modules.
    use crate::canvas::Canvas;
    use crate::color::Color;
    use crate::test_hack::expect_eq;
    use crate::test_hack::expect_float_eq;
    use crate::test_hack::setup;
    use crate::test_hack::teardown;

    // Ensures canvases can be properly created.
    #[test]
    fn test_create_canvas() {
        unsafe { setup() };
        let c = Canvas::new(10, 20);
        expect_eq!(c.width, 10);
        expect_eq!(c.height, 20);

        for column in c.pixels {
            for pixel in column {
                expect_eq!(pixel, Color::new(0.0, 0.0, 0.0));
            }
        }
        unsafe { teardown() };
    }

    // Ensures canvases can be written to
    #[test]
    fn test_write_pixel() {
        unsafe { setup() };
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.pixels[2][3] = red;
        expect_eq!(c.pixels[2][3], red);
        unsafe { teardown() };
    }
}
