use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::world::World;

pub struct Camera {
    hsize: u32,
    vsize: u32,
    field_of_view: f32,
    transform: Matrix,
    pixel_size: f32,
    half_width: f32,
    half_height: f32,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f32) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f32 / vsize as f32;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize as f32;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, px: u32, py: u32) -> Ray {
        let xoffset = (px as f32 + 0.5) * self.pixel_size;
        let yoffset = (py as f32 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform().inverse() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform().inverse() * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..(self.vsize - 1) {
            for x in 0..(self.hsize - 1) {
                let ray = self.ray_for_pixel(x, y);
                image.write_pixel(x as usize, y as usize, world.color_at(&ray));
            }
        }
        image
    }

    pub fn hsize(&self) -> u32 {
        self.hsize
    }

    pub fn vsize(&self) -> u32 {
        self.vsize
    }

    pub fn field_of_view(&self) -> f32 {
        self.field_of_view
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn pixel_size(&self) -> f32 {
        self.pixel_size
    }

    pub fn set_transform(mut self, new: Matrix) -> Self {
        self.transform = new;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::vector::Vector;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f32::consts::PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize(), 160 as u32);
        assert_eq!(c.vsize(), 120);
        assert_eq!(c.field_of_view(), std::f32::consts::PI / 2.0);
        assert_eq!(c.transform(), &Matrix::identity());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f32::consts::PI / 2.0);
        assert_eq!(c.pixel_size(), 0.01);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, std::f32::consts::PI / 2.0);
        assert_eq!(c.pixel_size(), 0.01);
    }

    #[test]
    fn constructing_ray_through_center_canvas() {
        let c = Camera::new(201, 101, std::f32::consts::PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_through_corner_canvas() {
        let c = Camera::new(201, 101, std::f32::consts::PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction(), Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_transformed_camera() {
        let c = Camera::new(201, 101, std::f32::consts::PI / 2.0).set_transform(
            Matrix::rotation_y(std::f32::consts::PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0),
        );
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), Point::new(0.0, 2.0, -5.0));
        let trt = 2.0_f32.sqrt() / 2.0;
        assert_eq!(r.direction(), Vector::new(trt, 0.0, -trt));
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let c = Camera::new(11, 11, std::f32::consts::PI / 2.0)
            .set_transform(Matrix::view_transform(from, to, up));
        let image = c.render(w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
