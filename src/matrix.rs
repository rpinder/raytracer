use crate::point::Point;
use crate::utils::fp_equal;
use crate::vector::Vector;
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct Matrix {
    row: u32,
    col: u32,
    grid: Vec<f32>,
}

impl Matrix {
    pub fn new(row: u32, col: u32) -> Matrix {
        Matrix {
            row,
            col,
            grid: vec![0.0; (row * col).try_into().unwrap()],
        }
    }

    pub fn new_filled(arr: &[&[f32]]) -> Matrix {
        let mut grid = vec![];
        for row in arr {
            for col in *row {
                grid.push(*col)
            }
        }
        let row = (*arr).len() as u32;
        let col = arr[0].len() as u32;
        Matrix { row, col, grid }
    }

    pub fn get(&self, row: u32, col: u32) -> f32 {
        if row > self.row {
            panic!("row out of bounds")
        };
        if col > self.col {
            panic!("col out of bounds")
        }
        let col = col as usize;
        self.grid[row as usize * self.col as usize + col]
    }

    pub fn set(&mut self, row: u32, col: u32, val: f32) {
        if row > self.row {
            panic!("row out of bounds")
        };
        if col > self.col {
            panic!("col out of bounds")
        };
        let col = col as usize;
        self.grid[row as usize * self.col as usize + col] = val;
    }

    pub fn transpose(self) -> Matrix {
        let mut m = Matrix::new(4, 4);
        for i in 0..4 {
            for j in 0..4 {
                m.set(j, i, self.get(i, j))
            }
        }
        m
    }

    pub fn identity() -> Matrix {
        Matrix::new_filled(&[
            &[1.0, 0.0, 0.0, 0.0],
            &[0.0, 1.0, 0.0, 0.0],
            &[0.0, 0.0, 1.0, 0.0],
            &[0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn determinant(&self) -> f32 {
        assert!(self.row == self.col);
        match self.row {
            2 => self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0),
            _ => {
                let mut det = 0.0;
                for i in 0..self.col {
                    det += self.get(0, i) * self.cofactor(0, i)
                }
                det
            }
        }
    }

    pub fn submatrix(&self, drow: u32, dcol: u32) -> Matrix {
        let mut m = Matrix::new(self.row - 1, self.col - 1);
        let mut index = 0;
        for (i, val) in self.grid.iter().enumerate() {
            if (i >= (drow * self.col).try_into().unwrap()
                && i < ((drow + 1) * self.col).try_into().unwrap())
                || (i as u32) % self.col == dcol
            {
                continue;
            }
            m.grid[index] = *val;
            index += 1;
        }
        m
    }

    pub fn minor(&self, drow: u32, dcol: u32) -> f32 {
        self.submatrix(drow, dcol).determinant()
    }

    pub fn cofactor(&self, drow: u32, dcol: u32) -> f32 {
        self.minor(drow, dcol) * if (drow + dcol) % 2 == 0 { 1.0 } else { -1.0 }
    }

    pub fn invertible(&self) -> bool {
        !fp_equal(self.determinant(), 0.0)
    }

    pub fn inverse(&self) -> Matrix {
        assert!(self.invertible());

        let mut m = Matrix::new(self.row, self.col);

        for row in 0..self.row {
            for col in 0..self.col {
                let c = self.cofactor(row, col);
                m.set(col, row, c / self.determinant());
            }
        }
        m
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
        let mut m = Matrix::identity();
        m.set(0, 3, x);
        m.set(1, 3, y);
        m.set(2, 3, z);
        m
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
        let mut m = Matrix::new(4, 4);
        m.set(0, 0, x);
        m.set(1, 1, y);
        m.set(2, 2, z);
        m.set(3, 3, 1.0);
        m
    }

    pub fn rotation_x(angle: f32) -> Matrix {
        Matrix::new_filled(&[
            &[1.0, 0.0, 0.0, 0.0],
            &[0.0, angle.cos(), -angle.sin(), 0.0],
            &[0.0, angle.sin(), angle.cos(), 0.0],
            &[0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(angle: f32) -> Matrix {
        Matrix::new_filled(&[
            &[angle.cos(), 0.0, angle.sin(), 0.0],
            &[0.0, 1.0, 0.0, 0.0],
            &[-angle.sin(), 0.0, angle.cos(), 0.0],
            &[0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z(angle: f32) -> Matrix {
        Matrix::new_filled(&[
            &[angle.cos(), -angle.sin(), 0.0, 0.0],
            &[angle.sin(), angle.cos(), 0.0, 0.0],
            &[0.0, 0.0, 1.0, 0.0],
            &[0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix {
        Matrix::new_filled(&[
            &[1.0, xy, xz, 0.0],
            &[yx, 1.0, yz, 0.0],
            &[zx, zy, 1.0, 0.0],
            &[0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
        let forward = (to - from).normalize();
        let up_norm = up.normalize();
        let left = forward.cross(&up_norm);
        let true_up = left.cross(&forward);

        let orientation = Matrix::new_filled(&[
            &[left.x, left.y, left.z, 0.0],
            &[true_up.x, true_up.y, true_up.z, 0.0],
            &[-forward.x, -forward.y, -forward.z, 0.0],
            &[0.0, 0.0, 0.0, 1.0],
        ]);
        orientation * Matrix::translation(-from.x, -from.y, -from.z)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Mul<Matrix> for Matrix {
    type Output = Self;
    fn mul(self, other: Matrix) -> Matrix {
        let mut m = Matrix::new(4, 4);
        for row in 0..4 {
            for col in 0..4 {
                let val = self.get(row, 0) * other.get(0, col)
                    + self.get(row, 1) * other.get(1, col)
                    + self.get(row, 2) * other.get(2, col)
                    + self.get(row, 3) * other.get(3, col);
                m.set(row, col, val);
            }
        }
        m
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix {
        let mut m = Matrix::new(4, 4);
        for row in 0..4 {
            for col in 0..4 {
                let val = self.get(row, 0) * other.get(0, col)
                    + self.get(row, 1) * other.get(1, col)
                    + self.get(row, 2) * other.get(2, col)
                    + self.get(row, 3) * other.get(3, col);
                m.set(row, col, val);
            }
        }
        m
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        let vals: Vec<f32> = vec![0, 1, 2, 3]
            .into_iter()
            .map(|x| {
                self.get(x, 0) * other.x
                    + self.get(x, 1) * other.y
                    + self.get(x, 2) * other.z
                    + self.get(x, 3) * 1.0
            })
            .collect();
        Point {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Mul<&Point> for &Matrix {
    type Output = Point;
    fn mul(self, other: &Point) -> Point {
        let vals: Vec<f32> = vec![0, 1, 2, 3]
            .into_iter()
            .map(|x| {
                self.get(x, 0) * other.x
                    + self.get(x, 1) * other.y
                    + self.get(x, 2) * other.z
                    + self.get(x, 3) * 1.0
            })
            .collect();
        Point {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        let vals: Vec<f32> = vec![0, 1, 2, 3]
            .into_iter()
            .map(|x| self.get(x, 0) * other.x + self.get(x, 1) * other.y + self.get(x, 2) * other.z)
            .collect();
        Vector {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Mul<&Vector> for &Matrix {
    type Output = Vector;
    fn mul(self, other: &Vector) -> Vector {
        let vals: Vec<f32> = vec![0, 1, 2, 3]
            .into_iter()
            .map(|x| self.get(x, 0) * other.x + self.get(x, 1) * other.y + self.get(x, 2) * other.z)
            .collect();
        Vector {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.row == other.row && self.col == other.col);
        for i in 0..self.grid.len() {
            if !fp_equal(self.grid[i], other.grid[i]) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix::new_filled(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[5.5, 6.5, 7.5, 8.5],
            &[9.0, 10.0, 11.0, 12.0],
            &[13.5, 14.5, 15.5, 16.5],
        ]);
        assert!(fp_equal(m.get(0, 0), 1.0));
        assert!(fp_equal(m.get(0, 3), 4.0));
        assert!(fp_equal(m.get(1, 0), 5.5));
        assert!(fp_equal(m.get(1, 2), 7.5));
        assert!(fp_equal(m.get(2, 2), 11.0));
        assert!(fp_equal(m.get(3, 0), 13.5));
        assert!(fp_equal(m.get(3, 2), 15.5));
    }

    #[test]
    fn a_2x2_matrix() {
        let m = Matrix::new_filled(&[&[-3.0, 5.0], &[1.0, -2.0]]);
        assert!(fp_equal(m.get(0, 0), -3.0));
        assert!(fp_equal(m.get(0, 1), 5.0));
        assert!(fp_equal(m.get(1, 0), 1.0));
        assert!(fp_equal(m.get(1, 1), -2.0));
    }

    #[test]
    fn a_3x3_matrix() {
        let m = Matrix::new_filled(&[&[-3.0, 5.0, 0.0], &[1.0, -2.0, -7.0], &[0.0, 1.0, 1.0]]);
        assert!(fp_equal(m.get(0, 0), -3.0));
        assert!(fp_equal(m.get(1, 1), -2.0));
        assert!(fp_equal(m.get(2, 2), 1.0));
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let m = Matrix::new_filled(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[5.0, 6.0, 7.0, 8.0],
            &[9.0, 8.0, 7.0, 6.0],
            &[5.0, 4.0, 3.0, 2.0],
        ]);
        let n = Matrix::new_filled(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[5.0, 6.0, 7.0, 8.0],
            &[9.0, 8.0, 7.0, 6.0],
            &[5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(m == n);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let m = Matrix::new_filled(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[5.0, 6.0, 7.0, 8.0],
            &[9.0, 8.0, 7.0, 6.0],
            &[5.0, 4.0, 3.0, 2.0],
        ]);
        let n = Matrix::new_filled(&[
            &[2.0, 3.0, 4.0, 5.0],
            &[6.0, 7.0, 8.0, 9.0],
            &[8.0, 7.0, 6.0, 5.0],
            &[4.0, 3.0, 2.0, 1.0],
        ]);
        assert!(m != n);
    }

    #[test]
    fn multiplying_two_matrices() {
        let m = Matrix::new_filled(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[5.0, 6.0, 7.0, 8.0],
            &[9.0, 8.0, 7.0, 6.0],
            &[5.0, 4.0, 3.0, 2.0],
        ]);
        let n = Matrix::new_filled(&[
            &[-2.0, 1.0, 2.0, 3.0],
            &[3.0, 2.0, 1.0, -1.0],
            &[4.0, 3.0, 6.0, 5.0],
            &[1.0, 2.0, 7.0, 8.0],
        ]);

        let x = Matrix::new_filled(&[
            &[20.0, 22.0, 50.0, 48.0],
            &[44.0, 54.0, 114.0, 108.0],
            &[40.0, 58.0, 110.0, 102.0],
            &[16.0, 26.0, 46.0, 42.0],
        ]);

        assert!(m * n == x);
    }

    #[test]
    fn matrix_multiplied_by_a_point() {
        let a = Matrix::new_filled(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[2.0, 4.0, 4.0, 2.0],
            &[8.0, 6.0, 4.0, 1.0],
            &[0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Point::new(1.0, 2.0, 3.0);
        assert!(
            a * b
                == Point {
                    x: 18.0,
                    y: 24.0,
                    z: 33.0,
                }
        );
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let m = Matrix::new_filled(&[
            &[0.0, 1.0, 2.0, 4.0],
            &[1.0, 2.0, 4.0, 8.0],
            &[2.0, 4.0, 8.0, 16.0],
            &[4.0, 8.0, 16.0, 32.0],
        ]);
        let n = Matrix::new_filled(&[
            &[0.0, 1.0, 2.0, 4.0],
            &[1.0, 2.0, 4.0, 8.0],
            &[2.0, 4.0, 8.0, 16.0],
            &[4.0, 8.0, 16.0, 32.0],
        ]);
        assert!(m == n * Matrix::identity());
    }

    #[test]
    fn transposing_a_matrix() {
        let m = Matrix::new_filled(&[
            &[0.0, 9.0, 3.0, 0.0],
            &[9.0, 8.0, 0.0, 8.0],
            &[1.0, 8.0, 5.0, 3.0],
            &[0.0, 0.0, 5.0, 8.0],
        ]);
        let res = Matrix::new_filled(&[
            &[0.0, 9.0, 1.0, 0.0],
            &[9.0, 8.0, 8.0, 0.0],
            &[3.0, 0.0, 5.0, 5.0],
            &[0.0, 8.0, 3.0, 8.0],
        ]);
        assert!(m.transpose() == res);
    }

    #[test]
    fn transposing_the_identity_matrix() {
        assert!(Matrix::identity().transpose() == Matrix::identity());
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let m = Matrix::new_filled(&[&[1.0, 5.0], &[-3.0, 2.0]]);
        assert!(fp_equal(m.determinant(), 17.0));
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let a = Matrix::new_filled(&[&[1.0, 5.0, 0.0], &[-3.0, 2.0, 7.0], &[0.0, 6.0, -3.0]]);
        let b = Matrix::new_filled(&[&[-3.0, 2.0], &[0.0, 6.0]]);
        assert!(a.submatrix(0, 2) == b);
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let a = Matrix::new_filled(&[
            &[-6.0, 1.0, 1.0, 6.0],
            &[-8.0, 5.0, 8.0, 6.0],
            &[-1.0, 0.0, 8.0, 2.0],
            &[-7.0, 1.0, -1.0, 1.0],
        ]);
        let b = Matrix::new_filled(&[&[-6.0, 1.0, 6.0], &[-8.0, 8.0, 6.0], &[-7.0, -1.0, 1.0]]);
        assert!(a.submatrix(2, 1) == b);
    }

    #[test]
    fn calculating_minor_3x3() {
        let a = Matrix::new_filled(&[&[3.0, 5.0, 0.0], &[2.0, -1.0, -7.0], &[6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);
        assert!(fp_equal(b.determinant(), 25.0));
        assert!(fp_equal(a.minor(1, 0), 25.0));
    }

    #[test]
    fn calculating_cofactor_of_3x3() {
        let a = Matrix::new_filled(&[&[3.0, 5.0, 0.0], &[2.0, -1.0, -7.0], &[6.0, -1.0, 5.0]]);
        assert!(fp_equal(a.minor(0, 0), -12.0));
        assert!(fp_equal(a.cofactor(0, 0), -12.0));
        assert!(fp_equal(a.minor(1, 0), 25.0));
        assert!(fp_equal(a.cofactor(1, 0), -25.0));
    }

    #[test]
    fn calculating_determinant_of_3x3() {
        let a = Matrix::new_filled(&[&[1.0, 2.0, 6.0], &[-5.0, 8.0, -4.0], &[2.0, 6.0, 4.0]]);
        assert!(fp_equal(a.cofactor(0, 0), 56.0));
        assert!(fp_equal(a.cofactor(0, 1), 12.0));
        assert!(fp_equal(a.cofactor(0, 2), -46.0));
        assert!(fp_equal(a.determinant(), -196.0));
    }

    #[test]
    fn calculating_determinant_of_4x4() {
        let a = Matrix::new_filled(&[
            &[-2.0, -8.0, 3.0, 5.0],
            &[-3.0, 1.0, 7.0, 3.0],
            &[1.0, 2.0, -9.0, 6.0],
            &[-6.0, 7.0, 7.0, -9.0],
        ]);
        assert!(fp_equal(a.cofactor(0, 0), 690.0));
        assert!(fp_equal(a.cofactor(0, 1), 447.0));
        assert!(fp_equal(a.cofactor(0, 2), 210.0));
        assert!(fp_equal(a.cofactor(0, 3), 51.0));
        assert!(fp_equal(a.determinant(), -4071.0));
    }

    #[test]
    fn invertible_matrix_for_invertibility() {
        let a = Matrix::new_filled(&[
            &[6.0, 4.0, 4.0, 4.0],
            &[5.0, 5.0, 7.0, 6.0],
            &[4.0, -9.0, 3.0, -7.0],
            &[9.0, 1.0, 7.0, -6.0],
        ]);
        assert!(fp_equal(a.determinant(), -2120.0));
        assert!(a.invertible());
    }

    #[test]
    fn noninvertible_matrix_for_invertibility() {
        let a = Matrix::new_filled(&[
            &[-4.0, 2.0, -2.0, -3.0],
            &[9.0, 6.0, 2.0, 6.0],
            &[0.0, -5.0, 1.0, -5.0],
            &[0.0, 0.0, 0.0, 0.0],
        ]);
        assert!(fp_equal(a.determinant(), 0.0));
        assert!(!a.invertible());
    }

    #[test]
    fn calculating_inverse_of_matrix() {
        let a = Matrix::new_filled(&[
            &[-5.0, 2.0, 6.0, -8.0],
            &[1.0, -5.0, 1.0, 8.0],
            &[7.0, 7.0, -6.0, -7.0],
            &[1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse();
        assert!(fp_equal(a.determinant(), 532.0));
        assert!(fp_equal(a.cofactor(2, 3), -160.0));
        assert!(fp_equal(b.get(3, 2), -160.0 / 532.0));
        assert!(fp_equal(a.cofactor(3, 2), 105.0));
        assert!(fp_equal(b.get(2, 3), 105.0 / 532.0));

        let c = Matrix::new_filled(&[
            &[0.21805, 0.45113, 0.24060, -0.04511],
            &[-0.80827, -1.45677, -0.44361, 0.52068],
            &[-0.07895, -0.22368, -0.05263, 0.19737],
            &[-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert!(b == c);
    }

    #[test]
    fn calculating_inverse_of_matrix2() {
        let a = Matrix::new_filled(&[
            &[8.0, -5.0, 9.0, 2.0],
            &[7.0, 5.0, 6.0, 1.0],
            &[-6.0, 0.0, 9.0, 6.0],
            &[-3.0, 0.0, -9.0, -4.0],
        ]);
        let b = Matrix::new_filled(&[
            &[-0.15385, -0.15385, -0.28205, -0.53846],
            &[-0.07692, 0.12308, 0.02564, 0.03077],
            &[0.35897, 0.35897, 0.43590, 0.92308],
            &[-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert!(a.inverse() == b);
    }

    #[test]
    fn calculating_inverse_of_matrix3() {
        let a = Matrix::new_filled(&[
            &[9.0, 3.0, 0.0, 9.0],
            &[-5.0, -2.0, -6.0, -3.0],
            &[-4.0, 9.0, 6.0, 4.0],
            &[-7.0, 6.0, 6.0, 2.0],
        ]);
        let b = Matrix::new_filled(&[
            &[-0.04074, -0.07778, 0.14444, -0.22222],
            &[-0.07778, 0.03333, 0.36667, -0.33333],
            &[-0.02901, -0.14630, -0.10926, 0.12963],
            &[0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert!(a.inverse() == b);
    }

    #[test]
    fn multiply_a_product_by_its_inverse() {
        let a = Matrix::new_filled(&[
            &[3.0, -9.0, 7.0, 3.0],
            &[3.0, -8.0, 2.0, -9.0],
            &[-4.0, 4.0, 4.0, 1.0],
            &[-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::new_filled(&[
            &[8.0, 2.0, 2.0, 2.0],
            &[3.0, -1.0, 7.0, 0.0],
            &[7.0, 0.0, 5.0, 4.0],
            &[6.0, -2.0, 0.0, 5.0],
        ]);
        let c = &a * &b;
        assert!(c * b.inverse() == a);
    }

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert!(transform * p == Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_inverse_of_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Point::new(-3.0, 4.0, 5.0);
        assert!(inv * p == Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        assert!(v == transform * v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert!(transform * p == Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert!(transform * v == Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Point::new(-4.0, 6.0, 8.0);
        assert!(inv * v == Point::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert!(transform * p == Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(std::f32::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_x(std::f32::consts::PI / 2.0);
        assert!(half_quarter * p == Point::new(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0));
        assert!(full_quarter * p == Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opoosite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(std::f32::consts::PI / 4.0);
        let inv = half_quarter.inverse();
        assert!(inv * p == Point::new(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(std::f32::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_y(std::f32::consts::PI / 2.0);
        assert!(half_quarter * p == Point::new(2.0_f32.sqrt() / 2.0, 0.0, 2.0_f32.sqrt() / 2.0));
        assert!(full_quarter * p == Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(std::f32::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_z(std::f32::consts::PI / 2.0);
        assert!(half_quarter * p == Point::new(-2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0, 0.0));
        assert!(full_quarter * p == Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert!(transform * p == Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert!(transform * p == Point::new(6.0, 3.0, 4.0));
    }
    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert!(transform * p == Point::new(2.0, 5.0, 4.0));
    }
    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert!(transform * p == Point::new(2.0, 7.0, 4.0));
    }
    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert!(transform * p == Point::new(2.0, 3.0, 6.0));
    }
    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert!(transform * p == Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(std::f32::consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert!(p2 == Point::new(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert!(p3 == Point::new(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert!(p4 == Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(std::f32::consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert!(t * p == Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn transform_matrix_for_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::identity());
    }

    #[test]
    fn transform_matrix_for_looking_positive_z() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_moves_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);

        let exp = Matrix::new_filled(&[
            &[-0.50709, 0.50709, 0.67612, -2.36643],
            &[0.76772, 0.60609, 0.12122, -2.82843],
            &[-0.35857, 0.59761, -0.71714, 0.0],
            &[0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(t, exp);
    }
}
