use crate::color::Color;
use crate::utils::fp_equal;

#[derive(Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Material) -> bool {
        if self.color != other.color {
            return false;
        }
        for (a, b) in [
            (self.ambient, other.ambient),
            (self.diffuse, other.diffuse),
            (self.specular, other.specular),
            (self.shininess, other.shininess),
        ] {
            if a != b {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_material() {
        let m = Material::default();
        assert!(m.color == Color::new(1.0, 1.0, 1.0));
        assert!(fp_equal(m.ambient, 0.1));
        assert!(fp_equal(m.diffuse, 0.9));
        assert!(fp_equal(m.specular, 0.9));
        assert!(fp_equal(m.shininess, 200.0));
    }
}
