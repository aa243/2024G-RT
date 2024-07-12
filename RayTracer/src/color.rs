use std::ops::Mul;
mod sup;
use sup::Interval;
use crate::File;
use std::io::Write;
use crate::Vec3;
use std::sync::{Arc, Mutex};
extern crate image;
use image::{DynamicImage, GenericImageView};
use std::path::Path;

use image::RgbImage;
/// the multi-sample write_color() function
pub fn write_color(pixel_color: [u8; 3], img: &Arc<Mutex<RgbImage>>, i: usize, j: usize) {
    let mut img_lock = img.lock().unwrap(); // Lock the mutex to access the image buffer.
    let pixel = img_lock.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb(pixel_color);
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
    pub fn random() -> Self {
        Self {
            r: rand::random::<f64>(),
            g: rand::random::<f64>(),
            b: rand::random::<f64>(),
        }
    }
    pub fn random_between(min: f64, max: f64) -> Self {
        Self {
            r: rand::random::<f64>() * (max - min) + min,
            g: rand::random::<f64>() * (max - min) + min,
            b: rand::random::<f64>() * (max - min) + min,
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

pub struct Image {
    img: DynamicImage,

}

impl Image {
    // 构造函数，通过文件路径读取jpg图片
    pub fn new(file_path: &str) -> Result<Self, String> {
        match image::open(&Path::new(file_path)) {
            Ok(img) => Ok(Self { img }),
            Err(e) => Err(format!("Failed to open image: {}", e)),
        }
    }

    pub fn height(&self) -> u32 {
        self.img.height()
    }

    pub fn width(&self) -> u32 {
        self.img.width()
    }

    // 提供一个接口：pixel_data，通过整数i和j得到图片在这一像素的颜色
    pub fn pixel_data(&self, i: u32, j: u32) -> Option<(u8, u8, u8)> {
        if i < self.img.width() && j < self.img.height() {
            let pixel = self.img.get_pixel(i, j).0; // 获取像素值
            Some((pixel[0], pixel[1], pixel[2])) // 返回RGB值
        } else {
            None // 超出图片范围，返回None
        }
    }
}