use crate::{
    color::Color,
    material::Material,
    matrix::Matrix,
    point::Point,
    point_light::PointLight,
    ray::{hit, Intersection, Ray},
    sphere::Sphere,
    world_intersection::WorldIntersection,
};

pub struct World {
    objects: Vec<Sphere>,
    light: PointLight,
}

impl World {
    pub fn new(objects: Vec<Sphere>, light: PointLight) -> World {
        World { objects, light }
    }

    pub fn objects(&self) -> &Vec<Sphere> {
        &self.objects
    }

    pub fn light(&self) -> &PointLight {
        &self.light
    }

    pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
        let mut inters: Vec<Intersection> = self
            .objects()
            .iter()
            .map(|x| ray.intersect(x))
            .flatten()
            .collect();
        inters.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        inters
    }

    pub fn shade_hit(&self, comps: &WorldIntersection) -> Color {
        let shadowed = self.is_shadowed(*comps.over_point());
        self.light.lighting(
            comps.inter().object().material(),
            *comps.point(),
            *comps.eye(),
            *comps.normal(),
            shadowed
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let inters = self.intersect_world(ray);
        match hit(inters) {
            Some(int) => self.shade_hit(&WorldIntersection::precompute(int, ray)),
            None => Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        let v = self.light.position() - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect_world(&r);

        if let Some(h) = hit(intersections) {
            h.t() < distance
        } else {
            false   
        }
    }
}

impl Default for World {
    fn default() -> World {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mat = Material::default()
            .set_color(Color::new(0.8, 1.0, 0.6))
            .set_diffuse(0.7)
            .set_specular(0.2);
        let s1 = Sphere::default().set_material(mat);

        let s2 = Sphere::default().set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        World::new(vec![s1, s2], light)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, vector::Vector, world_intersection::WorldIntersection};

    use super::*;

    #[test]
    fn creating_a_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mat = Material::default()
            .set_color(Color::new(0.8, 1.0, 0.6))
            .set_diffuse(0.7)
            .set_specular(0.2);
        let s1 = Sphere::default().set_material(mat);

        let s2 = Sphere::default().set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert!(w.light == light);
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

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects()[0].clone();
        let i = Intersection::new(4.0, shape);
        let comps = WorldIntersection::precompute(i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn shading_intersection_from_inside() {
        let wpre = World::default();
        let light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let w = World::new(wpre.objects().clone(), light);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects()[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = WorldIntersection::precompute(i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498))
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let w = World::default();
        let mut outer = w.objects()[0].clone();
        outer.material.ambient = 1.0;
        let mut inner = w.objects()[1].clone();
        inner.material.ambient = 1.0;

        let light = (*w.light()).clone();

        let w2 = World::new(vec![outer, inner.clone()], light);
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w2.color_at(&r);
        assert_eq!(c, inner.material().color)
    }

    #[test]
    fn no_shadow_when_nothing_colinear() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shadow_when_object_between() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_object_behind_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_object_behind_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let s1 = Sphere::default();
        let s2 = Sphere::default().set_transform(Matrix::translation(0.0, 0.0, 10.0));
        let w = World::new(vec![s1, s2.clone()], light);
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, s2);
        let comps = WorldIntersection::precompute(i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
