use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::vector::Vector;
use crate::utils::fp_equal;

#[derive(Clone, PartialEq)]
pub struct Sphere {
    matrix: Matrix,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            matrix: Matrix::identity(),
        }
    }

    pub fn transform(&self) -> &Matrix {
        &self.matrix
    }

    pub fn set_transform(&mut self, m: Matrix) {
        self.matrix = m;
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
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert!(fp_equal(xs.0.t(), 3.0));
        assert!(fp_equal(xs.1.t(), 7.0));
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = r.intersect(&s);
        assert!(xs.is_none());
    }
}

