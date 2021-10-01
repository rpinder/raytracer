use crate::color::Color;
use crate::point::Point;

pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert!(light.position() == position);
        assert!(light.intensity() == intensity);
    }
}
