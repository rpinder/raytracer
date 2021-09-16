use crate::tuple::Tuple;
use crate::utils::fp_equal;
use std::convert::TryInto;

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

    pub fn equal(a: &Matrix, b: &Matrix) -> bool {
        assert!(a.row == b.row && a.col == b.col);
        for i in 0..a.grid.len() {
            if !fp_equal(a.grid[i], b.grid[i]) {
                return false;
            }
        }
        true
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

    pub fn determinant(self) -> f32 {
        assert!(self.row == self.col);
        match self.row {
            2 => self.get(0,0) * self.get(1,1) - self.get(0,1) * self.get(1,0),
            _ => {
                let mut det = 0.0;
                for i in 0..self.col {
                    det = det + self.get(0, i) * self.cofactor(0, i)
                }
                det
            },
        }
    }

    pub fn submatrix(&self, drow: u32, dcol: u32) -> Matrix {
        let mut m = Matrix::new(self.row - 1, self.col - 1);
        let mut index = 0;
        for (i, val) in self.grid.iter().enumerate() {
            if (i >= (drow * self.col).try_into().unwrap() && i < ((drow + 1) * self.col).try_into().unwrap()) || (i as u32) % self.col == dcol {
                continue;
            }
            m.grid[index] = *val;
            index = index + 1;
        }
        m
    }

    pub fn minor(&self, drow: u32, dcol: u32) -> f32 {
        self.submatrix(drow, dcol).determinant()
    }

    pub fn cofactor(&self, drow: u32, dcol: u32) -> f32 {
        self.minor(drow, dcol) * if (drow + dcol) % 2 == 0 { 1.0 } else { -1.0 }
    }
}

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

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, other: Tuple) -> Tuple {
        let vals: Vec<f32> = vec![0, 1, 2, 3]
            .into_iter()
            .map(|x| {
                return self.get(x, 0) * other.x
                    + self.get(x, 1) * other.y
                    + self.get(x, 2) * other.z
                    + self.get(x, 3) * other.w;
            })
            .collect();
        Tuple {
            x: vals[0],
            y: vals[1],
            z: vals[2],
            w: vals[3],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::*;
    use crate::utils::fp_equal;

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
        assert!(Matrix::equal(&m, &n));
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
        assert!(!Matrix::equal(&m, &n));
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

        assert!(Matrix::equal(&(m * n), &x));
    }

    #[test]
    fn matrix_multiplied_by_a_tuple() {
        let a = Matrix::new_filled(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[2.0, 4.0, 4.0, 2.0],
            &[8.0, 6.0, 4.0, 1.0],
            &[0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };
        assert!(Tuple::equal(
            a * b,
            Tuple {
                x: 18.0,
                y: 24.0,
                z: 33.0,
                w: 1.0
            }
        ));
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
        assert!(Matrix::equal(&m, &(n * Matrix::identity())));
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
        assert!(Matrix::equal(&m.transpose(), &res));
    }

    #[test]
    fn transposing_the_identity_matrix() {
        assert!(Matrix::equal(
            &Matrix::identity().transpose(),
            &Matrix::identity()
        ));
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let m = Matrix::new_filled(&[
            &[1.0, 5.0],
            &[-3.0, 2.0],
        ]);
        assert!(fp_equal(m.determinant(), 17.0));
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let a = Matrix::new_filled(&[
            &[1.0, 5.0, 0.0],
            &[-3.0, 2.0, 7.0],
            &[0.0, 6.0, -3.0],
        ]);
        let b = Matrix::new_filled(&[
            &[-3.0, 2.0],
            &[0.0, 6.0],
        ]);
        assert!(Matrix::equal(&a.submatrix(0, 2), &b));
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let a = Matrix::new_filled(&[
            &[-6.0, 1.0, 1.0, 6.0],
            &[-8.0, 5.0, 8.0, 6.0],
            &[-1.0, 0.0, 8.0, 2.0],
            &[-7.0, 1.0, -1.0, 1.0],
        ]);
        let b = Matrix::new_filled(&[
            &[-6.0, 1.0, 6.0],
            &[-8.0, 8.0, 6.0],
            &[-7.0, -1.0, 1.0],
        ]);
        assert!(Matrix::equal(&a.submatrix(2, 1), &b));
    }

    #[test]
    fn calculating_minor_3x3() {
        let a = Matrix::new_filled(&[
            &[3.0, 5.0, 0.0],
            &[2.0, -1.0, -7.0],
            &[6.0, -1.0, 5.0],
        ]);
        let b = a.submatrix(1, 0);
        assert!(fp_equal(b.determinant(), 25.0));
        assert!(fp_equal(a.minor(1, 0), 25.0));
    }

    #[test]
    fn calculating_cofactor_of_3x3() {
        let a = Matrix::new_filled(&[
            &[3.0, 5.0, 0.0],
            &[2.0, -1.0, -7.0],
            &[6.0, -1.0, 5.0],
        ]);
        assert!(fp_equal(a.minor(0,0), -12.0));
        assert!(fp_equal(a.cofactor(0,0), -12.0));
        assert!(fp_equal(a.minor(1, 0), 25.0));
        assert!(fp_equal(a.cofactor(1, 0), -25.0));
    }

    #[test]
    fn calculating_determinant_of_3x3() {
        let a = Matrix::new_filled(&[
            &[1.0, 2.0, 6.0],
            &[-5.0, 8.0, -4.0],
            &[2.0, 6.0, 4.0],
        ]);
        assert!(fp_equal(a.cofactor(0,0), 56.0));
        assert!(fp_equal(a.cofactor(0,1), 12.0));
        assert!(fp_equal(a.cofactor(0,2), -46.0));
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
        assert!(fp_equal(a.cofactor(0,0), 690.0));
        assert!(fp_equal(a.cofactor(0,1), 447.0));
        assert!(fp_equal(a.cofactor(0,2), 210.0));
        assert!(fp_equal(a.cofactor(0,3), 51.0));
        assert!(fp_equal(a.determinant(), -4071.0));
    }
}
