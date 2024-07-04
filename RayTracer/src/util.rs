#[path ="./color.rs"] 
mod color;
pub use color::*;
#[path ="./sup.rs"]
mod sup;
pub use sup::*;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use rand::random;
use crate::File;
use std::io::Write;
use std::process::exit;

// Note that currently it cannot distinguish whether object is in front of the camera or behind the camera.
// pub fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64{
//     let oc = center - r.origin();
//     let a = r.direction().squared_length();
//     let h = oc.dot(&r.direction());
//     let c = oc.squared_length() - radius*radius;
//     let discriminant = h*h - a*c;
    
//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         return (h - discriminant.sqrt()) / a;
//     }
// }

static INFINITY: f64 = f64::INFINITY;
static PI: f64 = std::f64::consts::PI;

// pub fn ray_color(r: Ray, world: &Box<dyn Hittable>) -> Color {
//     let mut rec = HitRecord{p: Point3::zero(), normal: Vec3::zero(), t: 0.0, front_face: true};
//     let hit = world.hit(r, Interval::new(0.0, INFINITY), &mut rec);
//     if hit {
//         let N = (rec.normal + 1.0) * 0.5;

//         return Color::new((255.0 * N.x) as u8, (255.0 * N.y) as u8, (255.0 * N.z) as u8);
//     }

//     let direct = r.direction();
//     let a = 0.5 * (direct.y + 1.0);
//     let color1 = Color::new(255 as u8, 255 as u8,  255 as u8);
//     let color2 = Color::new(128,179,255);
//     return color1 * (1.0-a) + color2 * a;
// }

pub trait Hittable<'a>{
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool;
    fn display(&self);
    fn get_material(&self) -> Option<&'a dyn Material>;
}

#[derive(Clone, Copy)]
pub struct HitRecord<'a>{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<&'a dyn Material>, // Use the lifetime 'a here
}
impl<'a> HitRecord<'a>{
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool, mat: Option<&'a dyn Material>) -> Self {
        Self { p, normal, t, front_face, mat }
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {outward_normal * (-1.0)};
    }
}


#[derive(Clone, Copy)]
pub struct Sphere<'a>{
    center: Point3,
    radius: f64,
    mat: Option<&'a dyn Material>,
}

impl<'a> Sphere<'a>{
    pub fn new(center: Point3, radius: f64, mat: Option<&'a dyn Material>) -> Self {
        Self { center, radius, mat}
    }
}

impl<'a> Hittable<'a> for Sphere<'a>{
    fn hit(&self,ray: Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().squared_length();
        let half_b = oc.dot(&ray.direction());
        let c = oc.squared_length() - self.radius*self.radius;
    
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
    
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surround(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surround(root) {
                return false;
            }
        }
    
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = self.get_material();
    
        return true;
    }
    fn display(&self) {
        println!("center: {:?}, radius: {:?}", self.center, self.radius);
    }

    fn get_material(&self) -> Option<&'a dyn Material> {
        self.mat
    }
}

pub struct HittableList<'a>{
    pub objects: Vec<Box<dyn Hittable<'a>>>,
}

impl<'a> HittableList<'a>{
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
    pub fn add(&mut self, object: Box<dyn Hittable<'a>>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'a> Hittable<'a> for HittableList<'a>{
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool {
        let mut temp_rec = HitRecord::new(Point3::zero(), Vec3::zero(), 0.0, false, None);
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min,closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
    fn display(&self) {
        for object in self.objects.iter() {
            object.display();
        }
    }

    fn get_material(&self) -> Option<&'a dyn Material> {
        None
    }
}

pub struct Camera{
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_horizontal: Vec3,
    pixel_vertical: Vec3,
}

impl Camera{
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Self {
        Self { aspect_ratio: aspect_ratio, image_width: image_width, samples_per_pixel: samples_per_pixel, max_depth: max_depth, image_height: 0, center: Point3::zero(), pixel00_loc: Point3::zero(), pixel_horizontal: Vec3::zero(), pixel_vertical: Vec3::zero() }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height > 1 { self.image_height} else {1};

        let viewport_height = 2.0;
        let viewport_width = (self.image_width as f64 / self.image_height as f64) * viewport_height;
        let focal_length = 1.0;

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, -viewport_height, 0.0);
        self.pixel_horizontal = horizontal / self.image_width as f64;
        self.pixel_vertical = vertical / self.image_height as f64;

        self.center = Point3::zero();
        let viewport_upperleft = self.center - Vec3::new(0.0, 0.0, focal_length) - horizontal/2.0 - vertical/2.0;
        self.pixel00_loc = viewport_upperleft + self.pixel_horizontal/2.0 + self.pixel_vertical/2.0;
    }
    
    fn ray_color(r: Ray, world: &Box<dyn Hittable>, depth: u32) -> Color {
        if depth <= 0{
            return Color::new(0.0,0.0,0.0);
        }
        let mut rec = HitRecord{p: Point3::zero(), normal: Vec3::zero(), t: 0.0, front_face: true, mat: None};
        let hit = world.hit(r, Interval::new(0.001, INFINITY), &mut rec);
        if hit {
            // lambertian
            // let direct = rec.normal + Vec3::random_unit_vector();

            // basic
            // let direct = Vec3::random_on_hemisphere(&rec.normal);

            let mut scattered = Ray::new(Point3::zero(), Vec3::zero());
            let mut attenuation = Color::new(0.0,0.0,0.0);

            if rec.mat.unwrap().scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return attenuation .element_mul( Self::ray_color(scattered, world, depth - 1));
            }

            // let color = Vec3::new(1.0,1.0,1.0) + rec.normal;
            // let color = color * 0.5 * 255.0;

            // return Color::new(color.x as u16, color.y as u16, color.z as u16);
        }
    
        let direct = r.direction();
        let a = 0.5 * (direct.y + 1.0);
        let color1 = Color::new(1.0, 1.0,  1.0);
        let color2 = Color::new(0.5,0.7,1.0);
        return color1 * (1.0-a) + color2 * a;
    }

    fn is_ci() -> bool {
        option_env!("CI").unwrap_or_default() == "true"
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        // let offset = Vec3::new(0.0,0.0,0.0);
        let pixel_center = self.pixel00_loc + (self.pixel_horizontal * (i + offset.x)) + (self.pixel_vertical * (j + offset.y));
        let direct = (pixel_center - self.center).normalize();
        Ray::new(self.center, direct)
    }

    pub fn render (&mut self, world: &Box<dyn Hittable>, path: &str) {
        self.initialize();
        let bar: ProgressBar = if Self::is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let mut file = File::create(path).expect("Failed to create file");
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height).expect("Failed to write header");

        for j in 0..self.image_height as usize {
            for i in 0..self.image_width as usize {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color = pixel_color + Self::ray_color(r,&world,self.max_depth);
                }
                pixel_color = pixel_color / self.samples_per_pixel as f64;
                write_color(pixel_color.to_rgb(), &mut file);
                bar.inc(1);
            }
        }
        bar.finish();
    }
}

pub fn random_double() -> f64 {
    random::<f64>()
}

pub fn random_between(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub trait Material{
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian{
    albedo: Color,
}

impl Lambertian{
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian{
    fn scatter (&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        
        *scattered = Ray::new(hit_record.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal{
    albedo: Color,
    fuzz: f64,
}

impl Metal{
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz}
    }
}

impl Material for Metal{
    fn scatter (&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = r_in.direction().reflect(hit_record.normal).normalize();
        reflected = reflected - Vec3::random_unit_vector() * self.fuzz;
        // println!("{:?}", Vec3::random_unit_vector().length());
        // let reflected = hit_record.normal + Vec3::random_unit_vector();

        *scattered = Ray::new(hit_record.p, reflected);
        *attenuation = self.albedo;
        return scattered.direction().dot(&hit_record.normal) > 0.0;
    }
}


