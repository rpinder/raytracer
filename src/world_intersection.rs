use crate::{
    point::Point,
    ray::{Intersection, Ray},
    vector::Vector,
};

pub struct WorldIntersection {
    inter: Intersection,
    point: Point,
    eye: Vector,
    normal: Vector,
    inside: bool,
}

impl WorldIntersection {
    pub fn precompute(inter: Intersection, ray: &Ray) -> WorldIntersection {
        let point = ray.position(inter.t());
        let eye = -ray.direction();
        let normal = inter.object().normal_at(point);
        let inside = normal.dot(&eye) < 0.0;
        WorldIntersection {
            point,
            eye,
            normal: if inside { -normal } else { normal },
            inter,
            inside,
        }
    }

    pub fn inter(&self) -> &Intersection {
        &self.inter
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn eye(&self) -> &Vector {
        &self.eye
    }

    pub fn normal(&self) -> &Vector {
        &self.normal
    }

    pub fn inside(&self) -> bool {
        self.inside
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        point::Point,
        ray::{Intersection, Ray},
        sphere::Sphere,
        vector::Vector,
    };

    use super::*;

    #[test]
    fn precomputiong_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, shape);
        let comps = WorldIntersection::precompute(i.clone(), &r);
        assert_eq!(comps.inter().t(), i.t());
        assert_eq!(comps.point(), &Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eye(), &Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal(), &Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn when_intersection_occurs_on_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, shape);
        let comps = WorldIntersection::precompute(i, &r);
        assert!(!comps.inside())
    }

    #[test]
    fn when_intersection_occurs_on_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, shape);
        let comps = WorldIntersection::precompute(i, &r);
        assert!(comps.inside());
        assert_eq!(comps.point(), &Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eye(), &Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal(), &Vector::new(0.0, 0.0, -1.0));
    }
}
