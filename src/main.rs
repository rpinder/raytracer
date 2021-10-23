use raytracer::camera::Camera;
use raytracer::color::*;
use raytracer::material::*;
use raytracer::matrix::*;
use raytracer::point::*;
use raytracer::point_light::*;
use raytracer::sphere::*;
use raytracer::vector::Vector;
use raytracer::world::World;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let floormaterial = Material::default()
        .set_color(Color::new(1.0, 0.9, 0.9))
        .set_specular(0.0);

    let floor = Sphere::default()
        .set_transform(Matrix::scaling(10.0, 0.01, 10.0))
        .set_material(floormaterial.clone());

    let left_wall = Sphere::default()
        .set_transform(
            Matrix::translation(0.0, 0.0, 5.0)
                * Matrix::rotation_y(-std::f32::consts::PI / 4.0)
                * Matrix::rotation_x(std::f32::consts::PI / 2.0)
                * Matrix::scaling(10.0, 0.01, 10.0),
        )
        .set_material(floormaterial.clone());

    let right_wall = Sphere::default()
        .set_transform(
            Matrix::translation(0.0, 0.0, 5.0)
                * Matrix::rotation_y(std::f32::consts::PI / 4.0)
                * Matrix::rotation_x(std::f32::consts::PI / 2.0)
                * Matrix::scaling(10.0, 0.01, 10.0),
        )
        .set_material(floormaterial);

    let middle = Sphere::default()
        .set_transform(Matrix::translation(-0.5, 1.0, 0.5))
        .set_material(
            Material::default()
                .set_color(Color::new(0.1, 1.0, 0.5))
                .set_diffuse(0.7)
                .set_specular(0.3),
        );

    let right = Sphere::default()
        .set_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5))
        .set_material(
            Material::default()
                .set_color(Color::new(0.5, 1.0, 0.1))
                .set_diffuse(0.7)
                .set_specular(0.3),
        );

    let left = Sphere::default()
        .set_transform(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33))
        .set_material(
            Material::default()
                .set_color(Color::new(1.0, 0.8, 0.1))
                .set_diffuse(0.7)
                .set_specular(0.3),
        );

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let camera = Camera::new(500, 500, std::f32::consts::PI / 3.0)
        .set_transform(Matrix::view_transform(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ));

    let world = World::new(vec![floor, left_wall, right_wall, middle, right, left], light);
    let canvas = camera.render(world);

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
