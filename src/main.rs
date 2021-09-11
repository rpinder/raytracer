use raytracer::color::*;
use raytracer::utils::*;
use raytracer::tuple::*;
use raytracer::canvas::*;

fn main() {
    println!("Hello, world!");
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let pos = proj.position + proj.velocity;
    let vel = proj.velocity + env.gravity + env.wind;
    Projectile { position: pos, velocity: vel }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tuple_with_w1_is_a_point() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert!(fp_equal(a.x, 4.3));
        assert!(fp_equal(a.y, -4.2));
        assert!(fp_equal(a.z, 3.1));
        assert!(fp_equal(a.w, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w0_is_a_vector() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert!(fp_equal(a.x, 4.3));
        assert!(fp_equal(a.y, -4.2));
        assert!(fp_equal(a.z, 3.1));
        assert!(fp_equal(a.w, 0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w1() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert!(Tuple::equal(
            p,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0
            }
        ))
    }

    #[test]
    fn vector_creates_tuples_with_w0() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
        assert!(Tuple::equal(
            v,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0
            }
        ))
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let a2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        assert!(Tuple::equal(
            a1 + a2,
            Tuple {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 1.0
            }
        ))
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        assert!(Tuple::equal(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0)))
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        assert!(Tuple::equal(p - v, Tuple::point(-2.0, -4.0, -6.0)))
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        assert!(Tuple::equal(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0)))
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert!(Tuple::equal(zero - v, Tuple::vector(-1.0, 2.0, -3.0)))
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert!(Tuple::equal(
            -a,
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        ))
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert!(Tuple::equal(
            a * 3.5,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            }
        ))
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert!(Tuple::equal(
            a * 0.5,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        ))
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert!(Tuple::equal(
            a / 2.0,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        ))
    }

    #[test]
    fn magnitude_of_1_0_0_vector() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert!(fp_equal(v.magnitude(), 1.0))
    }

    #[test]
    fn magnitude_of_0_1_0_vector() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert!(fp_equal(v.magnitude(), 1.0))
    }

    #[test]
    fn magnitude_of_0_0_1_vector() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert!(fp_equal(v.magnitude(), 1.0))
    }

    #[test]
    fn magnitude_of_1_2_3_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(fp_equal(v.magnitude(), (14.0_f32).sqrt()))
    }

    #[test]
    fn magnitude_of_1_2_3_negate_vector() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert!(fp_equal(v.magnitude(), (14.0_f32).sqrt()))
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert!(Tuple::equal(v.normalize(), Tuple::vector(1.0, 0.0, 0.0)))
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let rt14 = 14.0_f32.sqrt();
        assert!(Tuple::equal(
            v.normalize(),
            Tuple::vector(1.0 / rt14, 2.0 / rt14, 3.0 / rt14)
        ))
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(fp_equal(v.normalize().magnitude(), 1.0))
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert!(fp_equal(a.dot(&b), 20.0))
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert!(Tuple::equal(a.cross(&b), Tuple::vector(-1.0, 2.0, -1.0)));
        assert!(Tuple::equal(b.cross(&a), Tuple::vector(1.0, -2.0, 1.0)))
    }

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert!(fp_equal(c.red, -0.5));
        assert!(fp_equal(c.green, 0.4));
        assert!(fp_equal(c.blue, 1.7));
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(Color::equal(c1 + c2, Color::new(1.6, 0.7, 1.0)));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(Color::equal(c1 - c2, Color::new(0.2, 0.5, 0.5)));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert!(Color::equal(c * 2.0, Color::new(0.4, 0.6, 0.8)));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert!(Color::equal(c1 * c2, Color::new(0.9, 0.2, 0.04)));
    }

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert!(c.width == 10 && c.height == 20);

        for line in c.grid {
            for pixel in line {
                assert!(Color::equal(pixel, Color::new(0.0, 0.0, 0.0)))
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);
        assert!(Color::equal(c.pixel_at(2, 3), red))
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.lines().collect::<Vec<_>>()[0..3] == vec!("P3", "5 3", "255"))
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        assert!(ppm.lines().collect::<Vec<_>>()[3..6] == vec!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"))
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, color);
            }
        }
        let ppm = c.to_ppm();
        println!("{}", ppm);
        assert!(ppm.lines().collect::<Vec<_>>()[3..7] == vec!("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204", "153 255 204 153 255 204 153 255 204 153 255 204 153", "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204", "153 255 204 153 255 204 153 255 204 153 255 204 153"));
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.ends_with('\n'));
    }
}
