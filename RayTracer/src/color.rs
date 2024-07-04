use std::ops::Mul;
mod sup;
use sup::Interval;
use crate::File;
use std::io::Write;
use crate::Vec3;

use image::RgbImage;
/// the multi-sample write_color() function
pub fn write_color(mut pixel_color: [u8; 3], file: &mut File) {
    let color_interval = Interval::new(0.0, 256.0);
    pixel_color[0] = color_interval.clamp(pixel_color[0] as f64) as u8;
    pixel_color[1] = color_interval.clamp(pixel_color[1] as f64) as u8;
    pixel_color[2] = color_interval.clamp(pixel_color[2] as f64) as u8;
    writeln!(file, "{} {} {}", pixel_color[0], pixel_color[1], pixel_color[2]).expect("Failed to write pixel data");
    // Write the translated [0,255] value of each color component.
}

#[derive (Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl Color{
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
    fn linear_to_gamma(x: f64) -> f64 {
        x.sqrt()
    }
    pub fn to_rgb(&self) -> [u8; 3] {
        [(Self::linear_to_gamma(self.r) * 255.0) as u8, (Self::linear_to_gamma(self.g) * 255.0) as u8, (Self::linear_to_gamma(self.b) * 255.0) as u8]
    }
    pub fn to_Vec3(&self) -> Vec3 {
        Vec3::new(self.r as f64, self.g as f64, self.b as f64)
    }
    pub fn element_mul(&self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl std::ops::Add for Color{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl std::ops::Add<Vec3> for Color{
    type Output = Self;

    fn add(self, other: Vec3) -> Self {
        Self {
            r: self.r + other.x,
            g: self.g + other.y,
            b: self.b + other.z,
        }
    }
}

impl std::ops::Div<f64> for Color{
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}
