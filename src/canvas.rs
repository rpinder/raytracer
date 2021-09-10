use std::convert::TryInto;
use crate::color::*;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            grid: vec![vec![Color::new(0.0, 0.0, 0.0); width.try_into().unwrap()]; height.try_into().unwrap()],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.grid[x][y]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.grid[x][y] = color;
    }
}
