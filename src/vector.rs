use crate::utils::fp_equal;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        let mut total: f32 = 0.0;
        for val in [self.x, self.y, self.z] {
            total += val.powi(2);
        }
        total.sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn reflect(&self, other: &Vector) -> Vector {
        *self - *other * 2.0 * self.dot(other)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        for (i, j) in [(self.x, other.x), (self.y, other.y), (self.z, other.z)] {
            if !fp_equal(i, j) {
                return false;
            }
        }
        true
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, divisor: f32) -> Self {
        self * (1.0 / divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_two_vectors() {
        let a1 = Vector {
            x: 3.0,
            y: -2.0,
            z: 5.0,
        };
        let a2 = Vector {
            x: -2.0,
            y: 3.0,
            z: 1.0,
        };
        assert!(
            a1 + a2
                == Vector {
                    x: 1.0,
                    y: 1.0,
                    z: 6.0,
                }
        )
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);
        assert!(zero - v == Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_vector() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert!(-a == Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert!(a * 3.5 == Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert!(a * 0.5 == Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        assert!(a / 2.0 == Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn magnitude_of_1_0_0_vector() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert!(fp_equal(v.magnitude(), 1.0))
    }

    #[test]
    fn magnitude_of_0_1_0_vector() {
        let v = Vector::new(0.0, 1.0, 0.0);
        assert!(fp_equal(v.magnitude(), 1.0))
    }

    #[test]
    fn magnitude_of_0_0_1_vector() {
        let v = Vector::new(0.0, 0.0, 1.0);
        assert!(fp_equal(v.magnitude(), 1.0))
    }

    #[test]
    fn magnitude_of_1_2_3_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert!(fp_equal(v.magnitude(), (14.0_f32).sqrt()))
    }

    #[test]
    fn magnitude_of_1_2_3_negate_vector() {
        let v = Vector::new(-1.0, -2.0, -3.0);
        assert!(fp_equal(v.magnitude(), (14.0_f32).sqrt()))
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = Vector::new(4.0, 0.0, 0.0);
        assert!(v.normalize() == Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let rt14 = 14.0_f32.sqrt();
        assert!(v.normalize() == Vector::new(1.0 / rt14, 2.0 / rt14, 3.0 / rt14));
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert!(fp_equal(v.normalize().magnitude(), 1.0))
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert!(fp_equal(a.dot(&b), 20.0))
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert!(a.cross(&b) == Vector::new(-1.0, 2.0, -1.0));
        assert!(b.cross(&a) == Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_vector_approaching_at_45() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert!(r == Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let x = 2.0_f32.sqrt() / 2.0;
        let n = Vector::new(x, x, 0.0);
        let r = v.reflect(&n);
        assert!(r == Vector::new(1.0, 0.0, 0.0));
    }
}
