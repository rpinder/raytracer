use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::point::*;
use raytracer::ray::*;
use raytracer::sphere::*;
use raytracer::material::*;
use raytracer::point_light::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10;
    let wall_size = 7;
    let canvas_pixels = 500;
    let pixel_size = wall_size as f32 / canvas_pixels as f32;
    let half = wall_size as f32 / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = Sphere::default();
    let mat = Material::default().set_color(Color::new(1.0, 0.2, 1.0));
    shape.set_material(mat);

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    for y in 0..canvas_pixels {
        println!("{}", y);
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = Point::new(world_x, world_y, wall_z as f32);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = r.intersect(&shape);

            if let Some(hit) = hit(xs) {
                let point = r.position(hit.t());
                let normal = hit.object().normal_at(point);
                let eye = -r.direction();
                let color = light.lighting(hit.object().material(), point, eye, normal);
                canvas.write_pixel(x as usize, y as usize, color);
            }
        }
    }

    let path = Path::new("output.ppm");
    let mut file = match File::create(&path) {
        Err(e) => panic!("couldn't create file: {}", e),
        Ok(file) => file,
    };

    match file.write_all(canvas.to_ppm().as_bytes()) {
        Err(e) => panic!("couldn't write file: {}", e),
        Ok(_) => println!("written file"),
    }
}
