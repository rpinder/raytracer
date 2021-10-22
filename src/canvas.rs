use crate::color::*;
use std::convert::TryInto;

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
            grid: vec![
                vec![Color::new(0.0, 0.0, 0.0); width.try_into().unwrap()];
                height.try_into().unwrap()
            ],
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
        if val < 0.0 {
            val = 0.0
        };
        if val > 255.0 {
            val = 255.0
        };
        val.round() as u32
    }

    pub fn to_ppm(self) -> String {
        let mut str = format!("P3\n{} {}\n255\n", self.width, self.height);
        for line in self.grid {
            let mut newline = String::from("");
            for pixel in line {
                let new = format!(
                    "{} {} {} ",
                    Self::convert(pixel.red),
                    Self::convert(pixel.green),
                    Self::convert(pixel.blue)
                );
                newline.push_str(&new);
            }
            if newline.len() > 70 {
                let mut start = 0;
                for _ in 0..(newline.len()/70) {
                    let pos = newline[start..(start+71)].rfind(' ').unwrap();
                    newline.replace_range((start + pos)..(start + pos + 1), "\n");
                    start = start + pos;
                }
            }

                
            str.push_str(&newline);
            str.pop();
            str.push_str(&"\n")
        }
        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert!(c.width == 10 && c.height == 20);

        for line in c.grid {
            for pixel in line {
                assert!(Color::equal(pixel, Color::new(0.0, 0.0, 0.0)))
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);
        assert!(Color::equal(c.pixel_at(2, 3), red))
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.lines().collect::<Vec<_>>()[0..3] == vec!("P3", "5 3", "255"))
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        assert!(
            ppm.lines().collect::<Vec<_>>()[3..6]
                == vec!(
                    "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                    "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                    "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
                )
        )
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, color);
            }
        }
        let ppm = c.to_ppm();
        println!("{}", ppm);
        assert!(
            ppm.lines().collect::<Vec<_>>()[3..7]
                == vec!(
                    "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                    "153 255 204 153 255 204 153 255 204 153 255 204 153",
                    "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                    "153 255 204 153 255 204 153 255 204 153 255 204 153"
                )
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.ends_with('\n'));
    }
}
