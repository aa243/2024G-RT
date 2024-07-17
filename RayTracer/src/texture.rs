use crate::util::Color;
use crate::util::Image;
use crate::util::Interval;
use crate::util::Perlin;
use crate::util::Point3;
use crate::util::Vec3;
use crate::Arc;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct Solid_Color {
    albedo: Color,
}

impl Solid_Color {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
    pub fn new_by_f64(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for Solid_Color {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.albedo
    }
}

pub struct Checker_Texture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
    inv_scale: f64,
}

impl Checker_Texture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>, scale: f64) -> Self {
        Self {
            odd,
            even,
            inv_scale: 1.0 / scale,
        }
    }
    pub fn new_by_color(odd: Color, even: Color, scale: f64) -> Self {
        Self {
            odd: Arc::new(Solid_Color::new(odd)),
            even: Arc::new(Solid_Color::new(even)),
            inv_scale: 1.0 / scale,
        }
    }
}

impl Texture for Checker_Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let xInteger = (self.inv_scale * p.x).floor() as i32;
        let yInteger = (self.inv_scale * p.y).floor() as i32;
        let zInteger = (self.inv_scale * p.z).floor() as i32;
        let is_even = (xInteger + yInteger + zInteger) % 2 == 0;
        return if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        };
    }
}

pub struct Image_Texture {
    image: Image,
}

impl Image_Texture {
    pub fn new(filepath: &str) -> Self {
        Self {
            image: Image::new(filepath).unwrap(),
        }
    }
}

impl Texture for Image_Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        if self.image.width() == 0 || self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let mut i = (u * self.image.width() as f64) as u32;
        let mut j = (v * self.image.height() as f64) as u32;
        if i == self.image.width() {
            i = i - 1;
        }
        if j == self.image.height() {
            j = j - 1;
        }
        // println!("i: {}, j: {}", i, j);
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        return Color::new(
            color_scale * pixel.unwrap().0 as f64,
            color_scale * pixel.unwrap().1 as f64,
            color_scale * pixel.unwrap().2 as f64,
        );
    }
}

pub struct Noise_Texture {
    noise: Perlin,
    scale: f64,
}

impl Noise_Texture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise_Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1.0 + (p.z * self.scale + 10.0 * self.noise.turb(p, 7)).sin())
    }
}
