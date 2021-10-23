use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::point::Point;

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
        let half_view = (field_of_view/2.0).tan();
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
    use crate::vector::Vector;
    use super::*;

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
        let r = c.ray_for_pixel(100,50);
        assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_through_corner_canvas() {
        let c = Camera::new(201, 101, std::f32::consts::PI / 2.0);
        let r = c.ray_for_pixel(0,0);
        assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction(), Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_transformed_camera() {
        let c = Camera::new(201, 101, std::f32::consts::PI / 2.0).set_transform(Matrix::rotation_y(std::f32::consts::PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0));
        let r = c.ray_for_pixel(100,50);
        assert_eq!(r.origin(), Point::new(0.0, 2.0, -5.0));
        let trt = 2.0_f32.sqrt() / 2.0;
        assert_eq!(r.direction(), Vector::new(trt, 0.0, -trt));
    }
}
