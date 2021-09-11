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

    pub fn set(mut self, row: u32, col: u32, val: f32) {
        if row > self.row {
            panic!("row out of bounds")
        };
        if col > self.col {
            panic!("col out of bounds")
        };
        let col = col as usize;
        self.grid[row as usize * self.col as usize + col] = val;
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
}
