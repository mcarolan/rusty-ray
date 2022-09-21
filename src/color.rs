use std::fmt::format;

use approx::AbsDiffEq;

pub struct Canvas {
    pixels: Vec<Color>,
    pub width: i64,
    pub height: i64,
}

struct PPMBuilder {
    result: String,
    current_line: String
}

impl PPMBuilder {
    fn new(width: i64, height: i64) -> PPMBuilder {
        PPMBuilder { result: format!("P3\n{} {}\n255\n", width, height), current_line: String::new() }
    }

    fn add_color(&mut self, color: &Color) {
        fn component(f: f64) -> i32 {
            ((f * 255.0).round() as i32).max(0).min(255)
        }

        self.add_component(component(color.red));
        self.add_component(component(color.green));
        self.add_component(component(color.blue));
    }

    fn add_component(&mut self, component: i32) {
        if self.current_line.is_empty() {
            self.current_line.push_str(&format!("{}", component));
        }
        else {
            let new_line_length = self.current_line.chars().count() + format!(" {}", component).chars().count();

            if new_line_length > 70 {
                self.force_next_line();
                self.current_line.push_str(&format!("{}", component));
            }
            else {
                self.current_line.push_str(&format!(" {}", component));
            }
        }
    }

    fn force_next_line(&mut self) {
        if !self.current_line.is_empty() {
            self.result.push_str(&self.current_line);
            self.result.push_str("\n");
            self.current_line = String::new();
        }
    }
}

impl Canvas {
    pub fn new(width: i64, height: i64) -> Canvas {
        let mut pixels = Vec::with_capacity((width * height).try_into().unwrap());

        for _i in 0..width {
            for _j in 0..height {
                pixels.push(Color::BLACK);
            }
        }
        Canvas {
            pixels: pixels,
            width: width,
            height: height,
        }
    }

    pub fn pixel_at(&self, x: i64, y: i64) -> Option<&Color> {
        let index: usize = (x + (y * self.width)).try_into().unwrap();
        self.pixels.get(index)
    }

    pub fn write_pixel(&mut self, x: i64, y: i64, color: &Color) -> Option<()> {
        let index: usize = (x + (y * self.width)).try_into().unwrap();
        let pixel = self.pixels.get_mut(index);
        pixel.map(|c| {
            *c = *color;
        })
    }

    pub fn generate_ppm(&self) -> String {
        let mut ppm_bulder = PPMBuilder::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                self.pixel_at(x, y).map(|p| {
                    ppm_bulder.add_color(p);
                });
            }
            ppm_bulder.force_next_line();
        }
        ppm_bulder.force_next_line();
        ppm_bulder.result
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl AbsDiffEq for Color {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        0.00001
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f64::abs_diff_eq(&self.red, &other.red, epsilon)
            && f64::abs_diff_eq(&self.green, &other.green, epsilon)
            && f64::abs_diff_eq(&self.blue, &other.blue, epsilon)
    }
}

impl Color {
    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }

    pub fn add(&self, other: &Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }

    pub fn subtract(&self, other: &Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }

    pub fn scalar_mul(&self, n: f64) -> Color {
        Color {
            red: self.red * n,
            green: self.green * n,
            blue: self.blue * n,
        }
    }

    pub fn mul(&self, other: &Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Canvas, Color};

    #[test]
    fn test_new() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_abs_diff_eq!(c.red, -0.5);
        assert_abs_diff_eq!(c.green, 0.4);
        assert_abs_diff_eq!(c.blue, 1.7);
    }

    #[test]
    fn test_color_operations() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_abs_diff_eq!(c1.add(&c2), Color::new(1.6, 0.7, 1.0));
        assert_abs_diff_eq!(c1.subtract(&c2), Color::new(0.2, 0.5, 0.5));
        assert_abs_diff_eq!(
            Color::new(0.2, 0.3, 0.4).scalar_mul(2.0),
            Color::new(0.4, 0.6, 0.8)
        );
    }

    #[test]
    fn test_color_multiply() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_abs_diff_eq!(c1.mul(&c2), Color::new(0.9, 0.2, 0.04))
    }

    #[test]
    fn test_canvas_new() {
        let c = Canvas::new(2, 2);
        assert_eq!(c.pixel_at(0, 0), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(1, 0), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(0, 1), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(1, 1), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(2, 1), None);
    }

    #[test]
    fn test_canvas_write_pixel() {
        let mut c = Canvas::new(2, 2);
        let red = Color::new(1.0, 0.0, 0.0);

        assert_eq!(c.pixel_at(0, 0), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(1, 0), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(0, 1), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(1, 1), Some(&Color::BLACK));
        c.write_pixel(1, 1, &red);

        assert_eq!(c.pixel_at(0, 0), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(1, 0), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(0, 1), Some(&Color::BLACK));
        assert_eq!(c.pixel_at(1, 1), Some(&red));
    }

    #[test]
    fn test_canvas_generate_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.generate_ppm();
        let actual: Vec<&str> = ppm.lines().take(3).collect();
        let expected = ["P3", "5 3", "255"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_canvas_generate_ppm_simple() {
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, &c1);
        c.write_pixel(2, 1, &c2);
        c.write_pixel(4, 2, &c3);

        let ppm = c.generate_ppm();
        let actual: Vec<&str> = ppm.lines().skip(3).take(3).collect();
        let expected = [
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_canvas_generate_ppm_long_lines() {
        let color = Color::new(1.0, 0.8, 0.6);

        let mut c = Canvas::new(10, 2);

        for x in 0..c.width {
            for y in 0..c.height {
                c.write_pixel(x, y, &color);
            }
        }

        let ppm = c.generate_ppm();
        let actual: Vec<&str> = ppm.lines().skip(3).take(4).collect();
        let expected = [
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_canvas_generate_ppm_ends_with_newline() {
        let mut c = Canvas::new(10, 2);
        let ppm = c.generate_ppm();
        assert!(ppm.ends_with("\n"));
    }
}
