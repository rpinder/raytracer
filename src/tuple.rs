use crate::utils::*;

#[derive(Copy, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn point(a: f32, b: f32, c: f32) -> Tuple {
        Tuple {
            x: a,
            y: b,
            z: c,
            w: 1.0,
        }
    }

    pub fn vector(a: f32, b: f32, c: f32) -> Tuple {
        Tuple {
            x: a,
            y: b,
            z: c,
            w: 0.0,
        }
    }

    pub fn is_point(&self) -> bool {
        fp_equal(self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        fp_equal(self.w, 0.0)
    }

    pub fn magnitude(&self) -> f32 {
        let mut total: f32 = 0.0;
        for val in [self.x, self.y, self.z, self.w] {
            total += val.powi(2);
        }
        total.sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: &Tuple) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn equal(a: Tuple, b: Tuple) -> bool {
        for (i, j) in [(a.x, b.x), (a.y, b.y), (a.z, b.z), (a.w, b.w)] {
            if !fp_equal(i, j) {
                return false;
            }
        }
        true
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl std::ops::Div<f32> for Tuple {
    type Output = Self;

    fn div(self, divisor: f32) -> Tuple {
        self * (1.0 / divisor)
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::*;
    
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
}






