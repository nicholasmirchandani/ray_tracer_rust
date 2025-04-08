mod test_hack;
#[path = "../src/color.rs"] mod color;

#[cfg(test)]
mod color_test {
    // Import the necessary modules.
    use crate::test_hack::expect_eq;
    use crate::test_hack::expect_float_eq;
    use crate::test_hack::setup;
    use crate::test_hack::teardown;
    use crate::color::Color;

    #[test]
    fn test_create_color() {
        unsafe { setup() };
        let c = Color {red: -0.5, green: 0.4, blue: 1.7};
        expect_float_eq!(c.red, -0.5);
        expect_float_eq!(c.green, 0.4);
        expect_float_eq!(c.blue, 1.7);
        unsafe { teardown() };
    }
}