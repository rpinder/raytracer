use crate::{color::Color, material::Material, matrix::Matrix, point::Point, point_light::PointLight, ray::{Intersection, Ray}, sphere::Sphere};

pub struct World {
    objects: Vec<Sphere>,
    lights: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Sphere>, lights: Vec<PointLight>) -> World {
        World {
            objects,
            lights,
        }
    }

    pub fn objects(&self) -> &Vec<Sphere> {
        &self.objects
    }

    pub fn lights(&self) -> &Vec<PointLight> {
        &self.lights
    }

    pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
        let mut inters: Vec<Intersection> = self.objects().iter().map(|x| ray.intersect(&x)).flatten().collect();
        inters.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        inters
    }
}

impl Default for World {
    fn default() -> World {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1 = Sphere::default();
        let mut mat = Material::default();
        mat.color = Color::new(0.8, 1.0, 0.6);
        mat.diffuse = 0.7;
        mat.specular = 0.2;
        s1.set_material(mat);

        let mut s2 = Sphere::default();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        World::new(vec![s1, s2], vec![light])
    }
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, vector::Vector};

    use super::*;

    #[test]
    fn creating_a_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1 = Sphere::default();
        let mut mat = Material::default();
        mat.color = Color::new(0.8, 1.0, 0.6);
        mat.diffuse = 0.7;
        mat.specular = 0.2;
        s1.set_material(mat);

        let mut s2 = Sphere::default();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert!(w.lights().contains(&light));
        assert!(w.objects().contains(&s1));
        assert!(w.objects().contains(&s2));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect_world(&r);

        assert!(xs.len() == 4);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 4.5);
        assert_eq!(xs[2].t(), 5.5);
        assert_eq!(xs[3].t(), 6.0);
    }
}
