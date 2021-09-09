struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
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

fn fp_equal(a: f32, b: f32) -> bool {
    let epsilon = 0.00001;
    f32::abs(a - b) < epsilon
}

fn tp_equal(a: Tuple, b: Tuple) -> bool {
    for (i, j) in [(a.x, b.x), (a.y, b.y), (a.z, b.z), (a.w, b.w)] {
        if !fp_equal(i, j) {
            return false;
        }
    }
    true
}

fn point(a: f32, b: f32, c: f32) -> Tuple {
    Tuple {
        x: a,
        y: b,
        z: c,
        w: 1.0,
    }
}

fn vector(a: f32, b: f32, c: f32) -> Tuple {
    Tuple {
        x: a,
        y: b,
        z: c,
        w: 0.0,
    }
}

fn main() {
    println!("Hello, world!");
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
        let p = point(4.0, -4.0, 3.0);
        assert!(tp_equal(
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
        let v = vector(4.0, -4.0, 3.0);
        assert!(tp_equal(
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
        assert!(tp_equal(
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
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert!(tp_equal(p1 - p2, vector(-2.0, -4.0, -6.0)))
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert!(tp_equal(p - v, point(-2.0, -4.0, -6.0)))
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert!(tp_equal(v1 - v2, vector(-2.0, -4.0, -6.0)))
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert!(tp_equal(zero - v, vector(-1.0, 2.0, -3.0)))
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert!(tp_equal(
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
        assert!(tp_equal(
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
        assert!(tp_equal(
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
        assert!(tp_equal(
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
        let v = vector(1.0, 0.0, 0.0);
        assert!(fp_equal(v.magnitude(), 1.0))
    }

    #[test]
    fn magnitude_of_0_1_0_vector() {
        let v = vector(0.0, 1.0, 0.0);
        assert!(fp_equal(v.magnitude(), 1.0))
    }

    #[test]
    fn magnitude_of_0_0_1_vector() {
        let v = vector(0.0, 0.0, 1.0);
        assert!(fp_equal(v.magnitude(), 1.0))
    }

    #[test]
    fn magnitude_of_1_2_3_vector() {
        let v = vector(1.0, 2.0, 3.0);
        assert!(fp_equal(v.magnitude(),  (14.0_f32).sqrt()))
    }

    #[test]
    fn magnitude_of_1_2_3_negate_vector() {
        let v = vector(-1.0, -2.0, -3.0);
        assert!(fp_equal(v.magnitude(), (14.0_f32).sqrt()))
    }

}
