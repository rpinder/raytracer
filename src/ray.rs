use crate::point::Point;
use crate::vector::Vector;
use crate::sphere::Sphere;
use crate::utils::fp_equal;

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

    pub fn intersect(&self, s: Sphere) -> Option<(Intersection, Intersection)> {
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

        Some((Intersection::new(t1, s), Intersection::new(t2, s)))
    }
}

#[derive(Clone, Copy)]
pub struct Intersection {
    t: f32,
    object: Sphere,
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Intersection {
        Intersection {t, object}
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn object(&self) -> Sphere {
        self.object
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

fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
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
        let xs = r.intersect(s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert!(fp_equal(xs.0.t, 4.0));
        assert!(fp_equal(xs.1.t, 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert!(fp_equal(xs.0.t, 5.0));
        assert!(fp_equal(xs.1.t, 5.0));
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(s);
        assert!(xs.is_none());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert!(fp_equal(xs.0.t, -1.0));
        assert!(fp_equal(xs.1.t, 1.0));
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert!(fp_equal(xs.0.t, -6.0));
        assert!(fp_equal(xs.1.t, -4.0));
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = intersections(&[i1, i2]);
        assert!(xs.len() == 2);
        assert!(fp_equal(xs[0].t(), 1.0));
        assert!(fp_equal(xs[1].t(), 2.0));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = r.intersect(s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert!(xs.0.object() == s);
        assert!(xs.1.object() == s);
    }

    #[test]
    fn hit_when_all_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = intersections(&[i2, i1]);
        let i = hit(xs);
        assert!(i == Some(i1));
    }

    #[test]
    fn hit_when_some_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = intersections(&[i2, i1]);
        let i = hit(xs);
        assert!(i == Some(i2));
    }

    #[test]
    fn hit_when_all_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = intersections(&[i2, i1]);
        let i = hit(xs);
        assert!(i == None);
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = intersections(&[i1, i2, i3, i4]);
        let i = hit(xs);
        assert!(i == Some(i4));
    }
}
