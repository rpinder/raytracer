use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::point::*;
use raytracer::ray::*;
use raytracer::sphere::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10;
    let wall_size = 7;
    let canvas_pixels = 100;
    let pixel_size = wall_size as f32 / canvas_pixels as f32;
    let half = wall_size as f32 / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let shape = Sphere::new();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = Point::new(world_x, world_y, wall_z as f32);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = r.intersect(&shape);

            if hit(xs).is_some() {
                canvas.write_pixel(x as usize, y as usize, color)
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
//     let start = Tuple::point(0.0, 1.0, 0.0);
//     let velocity = Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25;
//     let mut p = Projectile {
//         position: start,
//         velocity,
//     };

//     let gravity = Tuple::vector(0.0, -0.1, 0.0);
//     let wind = Tuple::vector(-0.01, 0.0, 0.0);
//     let e = Environment { gravity, wind };

//     let mut c = Canvas::new(900, 550);
//     let red = Color::new(1.0, 0.0, 0.0);

//     while p.position.y > 0.0 {
//         let x = p.position.x.round() as usize;
//         let y = (c.height - (p.position.y.round() as u32)) as usize;

//         c.write_pixel(x, y, red);
//         p = tick(&e, p);
//     }

//     let path = Path::new("output.ppm");
//     let mut file = match File::create(&path) {
//         Err(e) => panic!("couldn't create file: {}", e),
//         Ok(file) => file,
//     };

//     match file.write_all(c.to_ppm().as_bytes()) {
//         Err(e) => panic!("couldn't write file: {}", e),
//         Ok(_) => println!("written file"),
//     }

// struct Projectile {
//     position: Tuple,
//     velocity: Tuple,
// }

// struct Environment {
//     gravity: Tuple,
//     wind: Tuple,
// }

// fn tick(env: &Environment, proj: Projectile) -> Projectile {
//     let pos = proj.position + proj.velocity;
//     let vel = proj.velocity + env.gravity + env.wind;
//     Projectile {
//         position: pos,
//         velocity: vel,
//     }
// }
