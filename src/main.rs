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

fn fp_equal(a: f32, b: f32) -> bool {
    let epsilon = 0.00001;
    f32::abs(a - b) < epsilon
}

fn tp_equal(a: Tuple, b: Tuple) -> bool {
    for (i, j) in [(a.x, b.x), (a.y, b.y), (a.z, b.z), (a.w, b.w)] {
        if !fp_equal(i, j) {
            return false
        }
    }
    true
}

fn point(a: f32, b: f32, c: f32) -> Tuple {
    Tuple { x: a, y: b, z: c, w: 1.0 }
}

fn vector(a: f32, b: f32, c: f32) -> Tuple {
    Tuple { x: a, y: b, z: c, w: 0.0}
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tuple_with_w1_is_a_point() {
        let a = Tuple {x: 4.3, y: -4.2, z: 3.1, w: 1.0};
        assert!(fp_equal(a.x, 4.3));
        assert!(fp_equal(a.y, -4.2));
        assert!(fp_equal(a.z, 3.1));
        assert!(fp_equal(a.w, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w0_is_a_vector() {
        let a = Tuple {x: 4.3, y: -4.2, z: 3.1, w: 0.0};
        assert!(fp_equal(a.x, 4.3));
        assert!(fp_equal(a.y, -4.2));
        assert!(fp_equal(a.z, 3.1));
        assert!(fp_equal(a.w, 0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w1() {
        let p = point(4.0,-4.0,3.0);
        assert!(tp_equal(p, Tuple {x: 4.0, y: -4.0, z: 3.0, w: 1.0}))
    }

    #[test]
    fn vector_creates_tuples_with_w0() {
        let v = vector(4.0,-4.0,3.0);
        assert!(tp_equal(v, Tuple {x: 4.0, y: -4.0, z: 3.0, w: 0.0}))
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple {x: 3.0, y: -2.0, z: 5.0, w: 1.0};
        let a2 = Tuple {x: -2.0, y: 3.0, z: 1.0, w: 0.0};
        assert!(tp_equal(a1 + a2, Tuple {x: 1.0, y: 1.0, z: 6.0, w: 1.0}))
    }
}
