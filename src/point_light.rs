use crate::color::Color;
use crate::point::Point;
use crate::material::Material;
use crate::vector::Vector;

#[derive(PartialEq)]
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

    pub fn lighting(&self, mat: &Material, pos: Point, eye: Vector, normal: Vector) -> Color {
        let effective_color = mat.color * self.intensity;
        let lightv = (self.position - pos).normalize();
        let ambient = effective_color * mat.ambient;
        let light_dot_normal = lightv.dot(&normal);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            let black = Color::new(0.0, 0.0, 0.0);
            (black, black)
        } else {
            let diff = effective_color * mat.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(&normal);
            let reflect_dot_eye = reflectv.dot(&eye);
            if reflect_dot_eye <= 0.0 {
                (diff, Color::new(0.0, 0.0, 0.0))
            } else {
                let factor = reflect_dot_eye.powf(mat.shininess);
                let specular = self.intensity * mat.specular * factor;
                (diff, specular)
            }
        };
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert!(light.position() == position);
        assert!(light.intensity() == intensity);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.lighting(&m, position, eye, normal);
        assert!(result == Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_45_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let x = 2.0_f32.sqrt()/2.0;
        let eye = Vector::new(0.0, x, -x);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.lighting(&m, position, eye, normal);
        assert!(result == Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_45_offset() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let x = 2.0_f32.sqrt()/2.0;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.lighting(&m, position, eye, normal);
        assert!(result == Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let x = 2.0_f32.sqrt()/2.0;
        let eye = Vector::new(0.0, -x, -x);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.lighting(&m, position, eye, normal);
        assert!(result == Color::new(1.63639, 1.63639, 1.63639));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let x = 2.0_f32.sqrt()/2.0;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = light.lighting(&m, position, eye, normal);
        assert!(result == Color::new(0.1, 0.1, 0.1));
    }
}
