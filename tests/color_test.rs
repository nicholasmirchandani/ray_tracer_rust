#[path = "../src/color.rs"]
mod color;
mod test_hack;

#[cfg(test)]
mod color_test {

    // Import the necessary modules.
    use crate::color::Color;
    use crate::test_hack::expect_eq;
    use crate::test_hack::expect_float_eq;
    use crate::test_hack::setup;
    use crate::test_hack::teardown;

    // Ensures colors can be properly created.
    #[test]
    fn test_create_color() {
        unsafe { setup() };
        let c = Color::new(-0.5, 0.4, 1.7);
        expect_float_eq!(c.red, -0.5);
        expect_float_eq!(c.green, 0.4);
        expect_float_eq!(c.blue, 1.7);
        unsafe { teardown() };
    }

    // Ensures colors can be added.
    #[test]
    fn test_add_color() {
        unsafe { setup() };
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        expect_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
        unsafe { teardown() };
    }

    // Ensures colors can be subtracted.
    #[test]
    fn test_subtract_color() {
        unsafe { setup() };
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        expect_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
        unsafe { teardown() };
    }

    // Ensures scalar multiplication works as intended.
    #[test]
    fn test_scalar_multiply() {
        unsafe { setup() };
        let c = Color::new(0.2, 0.3, 0.4);
        expect_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
        unsafe { teardown() };
    }

    // Ensures color multiplication works as intended.
    #[test]
    fn test_color_multiply() {
        unsafe { setup() };
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        expect_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));

        unsafe { teardown() };
    }
}
