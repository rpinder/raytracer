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
        self.grid[y][x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.grid[y][x] = color;
    }

    fn convert(x: f32) -> u32 {
        let mut val = x * 255.0;
        if val < 0.0 { val = 0.0 };
        if val > 255.0 { val = 255.0 };
        val.round() as u32
    }

    pub fn to_ppm(self) -> String {
        let mut str = format!("P3\n{} {}\n255\n", self.width, self.height);
        for line in self.grid {
            let mut newline = String::from("");
            for pixel in line {
                let new = format!("{} {} {} ", Self::convert(pixel.red), Self::convert(pixel.green), Self::convert(pixel.blue));
                newline.push_str(&new);
            }
            if newline.len() > 70 {
                let pos = newline[0..70].rfind(' ').unwrap();
                newline.replace_range(pos..(pos+1), "\n");
            }
            str.push_str(&newline);
            str.pop();
            str.push_str(&"\n")
        }
        str
    }
}
