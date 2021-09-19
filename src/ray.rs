use crate::point::Point;
use crate::vector::Vector;

pub struct Sphere;

impl Sphere {
    pub fn new() -> Sphere {
        Sphere
    }
}

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

    pub fn intersect(&self, s: Sphere) -> Option<(f32, f32)> {
        let sphere_to_ray = self.origin - Point::new(0.0, 0.0, 0.0);
        let a = self.direction().dot(&self.direction());
        let b = 2.0 * self.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Some((t1, t2))
    }
}

mod tests {
    use crate::ray::*;
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
        let xs = r.intersect(s);
        assert!(xs != None);
        let xs = xs.unwrap();
        assert!(fp_equal(xs.0, 4.0));
        assert!(fp_equal(xs.1, 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(s);
        assert!(xs != None);
        let xs = xs.unwrap();
        assert!(fp_equal(xs.0, 5.0));
        assert!(fp_equal(xs.1, 5.0));
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(s);
        assert!(xs == None);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(s);
        assert!(xs != None);
        let xs = xs.unwrap();
        assert!(fp_equal(xs.0, -1.0));
        assert!(fp_equal(xs.1, 1.0));
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(s);
        assert!(xs != None);
        let xs = xs.unwrap();
        assert!(fp_equal(xs.0, -6.0));
        assert!(fp_equal(xs.1, -4.0));
    }
}
