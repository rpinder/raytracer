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

fn main() {
    println!("Hello, world!");
}

fn fp_equal(a: f32, b: f32) -> bool {
    let epsilon = 0.00001;
    f32::abs(a - b) < epsilon
}

#[cfg(test)]
mod tests {
    use crate::{Tuple, fp_equal};

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
}
