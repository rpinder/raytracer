pub fn fp_equal(a: f32, b: f32) -> bool {
    let epsilon = 0.00001;
    f32::abs(a - b) < epsilon
}
