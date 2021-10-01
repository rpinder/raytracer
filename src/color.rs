use crate::utils::fp_equal;

#[derive(Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn equal(a: Color, b: Color) -> bool {
        for (i, j) in [(a.red, b.red), (a.green, b.green), (a.blue, b.blue)] {
            if !fp_equal(i, j) {
                return false;
            }
        }
        true
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        for (i, j) in [
            (self.red, other.red),
            (self.green, other.green),
            (self.blue, other.blue),
        ] {
            if !fp_equal(i, j) {
                return false;
            }
        }
        true
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert!(fp_equal(c.red, -0.5));
        assert!(fp_equal(c.green, 0.4));
        assert!(fp_equal(c.blue, 1.7));
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(Color::equal(c1 + c2, Color::new(1.6, 0.7, 1.0)));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(Color::equal(c1 - c2, Color::new(0.2, 0.5, 0.5)));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert!(Color::equal(c * 2.0, Color::new(0.4, 0.6, 0.8)));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert!(Color::equal(c1 * c2, Color::new(0.9, 0.2, 0.04)));
    }
}
