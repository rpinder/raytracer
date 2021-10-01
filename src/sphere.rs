use crate::material::Material;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::utils::fp_equal;
use crate::vector::Vector;

#[derive(Clone, PartialEq)]
pub struct Sphere {
    matrix: Matrix,
    material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            matrix: Matrix::identity(),
            material: Material::default(),
        }
    }

    pub fn transform(&self) -> &Matrix {
        &self.matrix
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    pub fn set_transform(&mut self, m: Matrix) {
        self.matrix = m;
    }

    pub fn normal_at(&self, p: Point) -> Vector {
        let object_point = self.transform().inverse() * p;
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let world_normal = self.transform().inverse().transpose() * object_normal;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_default_implementation() {
        let s = Sphere::new();
        assert!(s.transform() == &Matrix::identity());
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        let t2 = t.clone();
        s.set_transform(t);
        assert!(s.transform() == &t2);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs = r.intersect(&s);
        assert!(fp_equal(xs[0].t(), 3.0));
        assert!(fp_equal(xs[1].t(), 7.0));
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = r.intersect(&s);
        assert!(xs.is_empty());
    }

    #[test]
    fn sphere_normal_at_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert!(n == Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn sphere_normal_at_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert!(n == Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn sphere_normal_at_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert!(n == Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn sphere_normal_at_nonaxial() {
        let s = Sphere::new();
        let x = 3.0_f32.sqrt() / 3.0;
        let n = s.normal_at(Point::new(x, x, x));
        assert!(n == Vector::new(x, x, x));
    }

    #[test]
    fn normal_is_normalized() {
        let s = Sphere::new();
        let x = 3.0_f32.sqrt() / 3.0;
        let n = s.normal_at(Point::new(x, x, x));
        assert!(n == n.normalize());
    }

    #[test]
    fn computinog_normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert!(n == Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        s.set_transform(
            Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f32::consts::PI / 5.0),
        );
        let x = 2.0_f32.sqrt();
        let n = s.normal_at(Point::new(0.0, x, -x));
        assert!(n == Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();
        let m = s.material;
        assert!(m == Material::default()); 
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = Sphere::new();
        let mut m = Material::default();
        m.ambient = 1.0;
        let m1 = m.clone();
        s.set_material(m);
        assert!(s.material() == &m1);
    }
}
