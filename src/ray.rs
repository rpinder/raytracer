use crate::matrix::Matrix;
use crate::point::Point;
use crate::sphere::Sphere;
use crate::utils::fp_equal;
use crate::vector::Vector;

#[derive(Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn position(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }

    pub fn intersect(&self, s: &Sphere) -> Vec<Intersection> {
        let ray = self.transform(s.transform().inverse());
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let s1 = s.clone();
        let s2 = s.clone();
        vec![Intersection::new(t1, s1), Intersection::new(t2, s2)]
    }

    pub fn transform(&self, m: Matrix) -> Ray {
        let origin = &m * &self.origin();
        let direction = &m * &self.direction();
        Ray { origin, direction }
    }
}

#[derive(Clone, Debug)]
pub struct Intersection {
    t: f32,
    object: Sphere,
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn object(&self) -> &Sphere {
        &self.object
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        fp_equal(self.t(), other.t())
    }
}

fn intersections(inters: &[Intersection]) -> Vec<Intersection> {
    inters.to_vec()
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let above_zero = intersections.iter().filter(|x| x.t() > 0.0);
    let mut current = std::f32::MAX;
    let mut cinter: Option<Intersection> = None;
    above_zero.into_iter().for_each(|inter| {
        if inter.t() < current {
            current = inter.t();
            cinter = Some(inter.clone());
        }
    });
    cinter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fp_equal;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert!(r.origin() == origin);
        assert!(r.direction() == direction);
    }

    #[test]
    fn computing_point_from_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert!(r.position(0.0) == Point::new(2.0, 3.0, 4.0));
        assert!(r.position(1.0) == Point::new(3.0, 3.0, 4.0));
        assert!(r.position(-1.0) == Point::new(1.0, 3.0, 4.0));
        assert!(r.position(2.5) == Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(&s);
        assert!(fp_equal(xs[0].t, 4.0));
        assert!(fp_equal(xs[1].t, 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(&s);
        assert!(fp_equal(xs[0].t, 5.0));
        assert!(fp_equal(xs[1].t, 5.0));
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(&s);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(&s);
        assert!(fp_equal(xs[0].t, -1.0));
        assert!(fp_equal(xs[1].t, 1.0));
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(&s);
        assert!(fp_equal(xs[0].t, -6.0));
        assert!(fp_equal(xs[1].t, -4.0));
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let s2 = s.clone();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s2);
        let xs = intersections(&[i1, i2]);
        assert!(xs.len() == 2);
        assert!(fp_equal(xs[0].t(), 1.0));
        assert!(fp_equal(xs[1].t(), 2.0));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(&s);
        assert!(xs[0].object() == &s);
        assert!(xs[1].object() == &s);
    }

    #[test]
    fn hit_when_all_positive_t() {
        let s = Sphere::new();
        let s2 = s.clone();
        let i1 = Intersection::new(1.0, s);
        let i1c = i1.clone();
        let i2 = Intersection::new(2.0, s2);
        let xs = intersections(&[i2, i1c]);
        let i = hit(xs);
        assert!(i == Some(i1.clone()));
    }

    #[test]
    fn hit_when_some_negative_t() {
        let s = Sphere::new();
        let s2 = s.clone();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s2);
        let i2c = i2.clone();
        let xs = intersections(&[i2c, i1]);
        let i = hit(xs);
        assert!(i == Some(i2));
    }

    #[test]
    fn hit_when_all_negative_t() {
        let s = Sphere::new();
        let s2 = s.clone();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s2);
        let xs = intersections(&[i2, i1]);
        let i = hit(xs);
        assert!(i == None);
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let s2 = s.clone();
        let s3 = s.clone();
        let s4 = s.clone();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s2);
        let i3 = Intersection::new(-3.0, s3);
        let i4 = Intersection::new(2.0, s4);
        let i4c = i4.clone();
        let xs = intersections(&[i1, i2, i3, i4c]);
        let i = hit(xs);
        assert!(i == Some(i4));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert!(r2.origin() == Point::new(4.0, 6.0, 8.0));
        assert!(r2.direction() == Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert!(r2.origin() == Point::new(2.0, 6.0, 12.0));
        assert!(r2.direction() == Vector::new(0.0, 3.0, 0.0));
    }
}
