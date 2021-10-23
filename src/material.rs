use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn set_color(mut self, new: Color) -> Self {
        self.color = new;
        self
    }

    pub fn set_ambient(mut self, new: f32) -> Self {
        self.ambient = new;
        self
    }

    pub fn set_diffuse(mut self, new: f32) -> Self {
        self.diffuse = new;
        self
    }

    pub fn set_specular(mut self, new: f32) -> Self {
        self.specular = new;
        self
    }

    pub fn set_shininess(mut self, new: f32) -> Self {
        self.shininess = new;
        self
    }
}

impl Default for Material {
    fn default() -> Material {
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
    use crate::utils::fp_equal;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert!(m.color == Color::new(1.0, 1.0, 1.0));
        assert!(fp_equal(m.ambient, 0.1));
        assert!(fp_equal(m.diffuse, 0.9));
        assert!(fp_equal(m.specular, 0.9));
        assert!(fp_equal(m.shininess, 200.0));
    }
}
