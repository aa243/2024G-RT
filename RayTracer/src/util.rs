#[path = "./color.rs"]
mod color;
pub use color::*;
#[path = "./texture.rs"]
mod texture;
pub use texture::*;
#[path = "./perlin.rs"]
mod perlin;
pub use perlin::Perlin;
#[path = "./plane.rs"]
mod plane;
pub use plane::*;
#[path = "./material.rs"]
mod material;
#[path = "./sup.rs"]
mod sup;
use crate::File;
use crossbeam::thread;
use image::ImageBuffer;
use indicatif::ProgressBar;
pub use material::*;
use rand::random;
use std::f64::consts::PI;
use std::sync::atomic::Ordering;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::{atomic::AtomicUsize, Arc};
pub use sup::*; //接收render传回来的图片，在main中文件输出

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

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn display(&self);
    fn get_material(&self) -> Option<Arc<dyn Material>>;
    fn bounding_box(&self) -> AABB;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>, // Change the lifetime to 'static
    pub u: f64,
    pub v: f64,
}
impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        mat: Option<Arc<dyn Material>>,
        u: f64,
        v: f64,
    ) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
            mat,
            u,
            v,
        }
    }
    pub fn default() -> Self {
        Self {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: None,
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * (-1.0)
        };
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Option<Arc<dyn Material>>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: AABB,
}

impl Sphere {
    pub fn new(
        center: Point3,
        radius: f64,
        mat: Option<Arc<dyn Material>>,
        center2: Point3,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::new_by_point(center - rvec, center + rvec);
        let box2 = AABB::new_by_point(center2 - rvec, center2 + rvec);
        Self {
            center,
            radius,
            mat,
            is_moving: true,
            center_vec: center2 - center,
            bbox: AABB::new_by_aabb(box1, box2),
        }
    }
    pub fn new_static(center: Point3, radius: f64, mat: Option<Arc<dyn Material>>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::zero(),
            bbox: AABB::new_by_point(center - rvec, center + rvec),
        }
    }
    pub fn sphere_center(&self, time: f64) -> Point3 {
        return self.center + self.center_vec * time;
    }
    fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
        let phi = (-p.z).atan2(p.x) + PI;
        let theta = (-p.y).acos();
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.sphere_center(ray.time())
        } else {
            self.center
        };
        let oc = center - ray.origin();
        let a = ray.direction().squared_length();
        let half_b = oc.dot(&ray.direction());
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (half_b - sqrtd) / a;
        if !ray_t.surround(root) {
            root = (half_b + sqrtd) / a;
            if !ray_t.surround(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        Self::get_sphere_uv(outward_normal.to_point3(), &mut rec.u, &mut rec.v);
        rec.mat = self.get_material();

        return true;
    }
    fn display(&self) {
        println!("center: {:?}, radius: {:?}", self.center, self.radius);
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        self.mat.as_ref().map(Arc::clone)
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::new_by_aabb(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(Point3::zero(), Vec3::zero(), 0.0, false, None, 0.0, 0.0);
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }
    fn display(&self) {
        for object in self.objects.iter() {
            object.display();
        }
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        None
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::default();
        for object_index in start..end {
            bbox = AABB::new_by_aabb(bbox, objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();
        let comparator = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            2 => BvhNode::box_z_compare,
            _ => panic!("Invalid axis"),
        };
        let object_span = end - start;
        // println!("start: {}, end: {}", start, end);
        // println!("object_span: {:?}", object_span);
        let mut left: Arc<dyn Hittable> = objects[start].clone();
        let mut right: Arc<dyn Hittable> = objects[start].clone();

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            let comparator_closure =
                |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| -> std::cmp::Ordering {
                    if comparator(a, b) {
                        std::cmp::Ordering::Less
                    } else if comparator(b, a) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                };
            objects[start..end].sort_unstable_by(comparator_closure);
            let mid = start + object_span / 2;
            left = Arc::new(Self::new(objects, start, mid));
            right = Arc::new(Self::new(objects, mid, end));
        }

        Self {
            left: left,
            right: right,
            bbox: bbox,
        }
    }
    pub fn new_by_object_list(list: &HittableList) -> Self {
        let mut objects = list.objects.clone();
        let number = objects.len();
        // println!("number: {:?}", number);
        Self::new(&mut objects, 0, number)
    }
    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> bool {
        let box_a_interval = a.bounding_box().axis_interval(axis_index);
        let box_b_interval = b.bounding_box().axis_interval(axis_index);
        return box_a_interval.min < box_b_interval.min;
    }
    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );
        return hit_left || hit_right;
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn display(&self) {
        println!("BvhNode");
    }
    fn get_material(&self) -> Option<Arc<dyn Material>> {
        None
    }
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box();
        Self {
            object,
            offset,
            bbox: bbox + offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if !self.object.hit(offset_r, ray_t, rec) {
            return false;
        }
        rec.p = rec.p + self.offset;
        return true;
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn display(&self) {
        println!("Translate");
    }
    fn get_material(&self) -> Option<Arc<dyn Material>> {
        None
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        if tester.iloc(c) > max.iloc(c) {
                            max.iloc_mut(c, tester.iloc(c));
                        }
                        if tester.iloc(c) < min.iloc(c) {
                            min.iloc_mut(c, tester.iloc(c));
                        }
                    }
                }
            }
        }
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::new_by_point(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = r.origin();
        let direction = r.direction();
        let origin_x = self.cos_theta * origin.x - self.sin_theta * origin.z;
        let origin_z = self.sin_theta * origin.x + self.cos_theta * origin.z;
        let direction_x = self.cos_theta * direction.x - self.sin_theta * direction.z;
        let direction_z = self.sin_theta * direction.x + self.cos_theta * direction.z;
        let rotated_r = Ray::new(
            Point3::new(origin_x, origin.y, origin_z),
            Vec3::new(direction_x, direction.y, direction_z),
            r.time(),
        );
        if !self.object.hit(rotated_r, ray_t, rec) {
            return false;
        }
        let p = rec.p;
        let normal = rec.normal;
        let p_x = self.cos_theta * p.x + self.sin_theta * p.z;
        let p_z = -self.sin_theta * p.x + self.cos_theta * p.z;
        let normal_x = self.cos_theta * normal.x + self.sin_theta * normal.z;
        let normal_z = -self.sin_theta * normal.x + self.cos_theta * normal.z;
        rec.p = Point3::new(p_x, p.y, p_z);
        rec.normal = Vec3::new(normal_x, normal.y, normal_z);
        return true;
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn display(&self) {
        println!("RotateY");
    }
    fn get_material(&self) -> Option<Arc<dyn Material>> {
        None
    }
}

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Option<Arc<dyn Material>>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new_by_color(boundary: Arc<dyn Hittable>, density: f64, color: Color) -> Self {
        Self {
            boundary,
            phase_function: Some(Arc::new(Isotropic::new_by_color(color)) as Arc<dyn Material>),
            neg_inv_density: -1.0 / density,
        }
    }
    pub fn new_by_tex(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            phase_function: Some(Arc::new(Isotropic::new(tex)) as Arc<dyn Material>),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();
        if !self
            .boundary
            .hit(r, Interval::new(-INFINITY, INFINITY), &mut rec1)
        {
            return false;
        }
        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = self.phase_function.clone();
        return true;
    }
    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
    fn display(&self) {
        println!("ConstantMedium");
    }
    fn get_material(&self) -> Option<Arc<dyn Material>> {
        self.phase_function.clone()
    }
}

// AABB
#[derive(Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}
impl AABB {
    pub fn default() -> Self {
        Self {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let ret = Self { x, y, z };
        ret.pad_to_minimums()
    }
    pub fn new_by_point(a: Point3, b: Point3) -> Self {
        let ret = Self {
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        };
        ret.pad_to_minimums()
    }
    pub fn new_by_aabb(a: AABB, b: AABB) -> Self {
        Self {
            x: Interval::new_by_interval(a.x, b.x),
            y: Interval::new_by_interval(a.y, b.y),
            z: Interval::new_by_interval(a.z, b.z),
        }
    }
    pub fn axis_interval(&self, axis: usize) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis"),
        }
    }
    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            return if self.x.size() > self.z.size() { 0 } else { 2 };
        } else {
            return if self.y.size() > self.z.size() { 1 } else { 2 };
        }
    }
    pub fn hit(&self, r: Ray, ray_t: Interval) -> bool {
        let mut ray_t = ray_t;
        let ray_origin = r.origin();
        let ray_direct = r.direction();
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_direct.iloc(axis);

            let t0 = (ax.min - ray_origin.iloc(axis)) * adinv;
            let t1 = (ax.max - ray_origin.iloc(axis)) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
    fn pad_to_minimums(mut self) -> Self {
        let delta = 0.00001;

        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
        self
    }
}

impl std::ops::Add<Vec3> for AABB {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background: Color,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_horizontal: Vec3,
    pixel_vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    sqrt_spp: i32,
    recip_sqrt_spp: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
        background: Color,
    ) -> Self {
        Self {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,
            vfov: vfov,
            lookfrom: lookfrom,
            lookat: lookat,
            vup: vup,
            defocus_angle: defocus_angle,
            focus_dist: focus_dist,
            background,
            image_height: 0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_horizontal: Vec3::zero(),
            pixel_vertical: Vec3::zero(),
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
            sqrt_spp: 0,
            recip_sqrt_spp: 0.0,
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height > 1 {
            self.image_height
        } else {
            1
        };
        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as i32;
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f64;
        self.center = self.lookfrom;

        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = (self.image_width as f64 / self.image_height as f64) * viewport_height;

        let horizontal = self.u * viewport_width;
        let vertical = self.v * (-1.0) * viewport_height;
        self.pixel_horizontal = horizontal / self.image_width as f64;
        self.pixel_vertical = vertical / self.image_height as f64;

        let viewport_upperleft =
            self.center - self.w * self.focus_dist - horizontal / 2.0 - vertical / 2.0;
        self.pixel00_loc =
            viewport_upperleft + self.pixel_horizontal / 2.0 + self.pixel_vertical / 2.0;

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: Ray, world: &Arc<dyn Hittable>, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: true,
            mat: None,
            u: 0.0,
            v: 0.0,
        };
        let hit = world.hit(r, Interval::new(0.001, INFINITY), &mut rec);
        if !hit {
            // let direction = r.direction().normalize();
            // let a = 0.5 * (direction.y + 1.0);
            // let color1 = Color::new(0.1, 0.1, 0.45);
            // let color2 = Color::new(0.2, 0.2, 0.9);
            // return color1 * (1.0 - a) + color2 * a;

            // Color::new(0.0, 0.0, 0.0)
            return self.background;
        } else {
            let mut scattered = Ray::new(Point3::zero(), Vec3::zero(), 0.0);
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            let rec_clone = rec.clone();
            let color_from_emission = rec_clone.mat.unwrap().emitted(rec.u, rec.v, &rec.p);
            let rec_clone = rec.clone();

            if rec
                .mat
                .unwrap()
                .scatter(&r, &rec_clone, &mut attenuation, &mut scattered)
            {
                let color_from_scatter =
                    attenuation.element_mul(self.ray_color(scattered, world, depth - 1));
                return color_from_emission + color_from_scatter;
            }

            return color_from_emission;
        }
    }

    fn is_ci() -> bool {
        option_env!("CI").unwrap_or_default() == "true"
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn get_ray(&self, i: f64, j: f64, s_i: i32, s_j: i32) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        // let offset = Vec3::new(0.0,0.0,0.0);
        let pixel_center = self.pixel00_loc
            + (self.pixel_horizontal * (i + offset.x))
            + (self.pixel_vertical * (j + offset.y));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direct = (pixel_center - ray_origin).normalize();
        let ray_time = random_double();
        Ray::new(ray_origin, ray_direct, ray_time)
    }

    fn sample_square_stratified(&self, s_i: i32, s_j: i32) -> Vec3 {
        let px = ((s_i as f64 + random_double()) * self.recip_sqrt_spp) as f64 - 0.5;
        let py = ((s_j as f64 + random_double()) * self.recip_sqrt_spp) as f64 - 0.5;
        Vec3::new(px, py, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }

    // pub fn render(&mut self, world: &Arc<dyn Hittable>, path: &str) {
    //     self.initialize();
    // let bar: ProgressBar = if Self::is_ci() {
    //     let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(*img.clone());e_height * self.image_width) as u64)
    // };

    //     let mut file = File::create(path).expect("Failed to create file");
    //     writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)
    //         .expect("Failed to write header");

    //     for j in 0..self.image_height as usize {
    //         for i in 0..self.image_width as usize {
    //             // let pixel_color: Color = (0..self.samples_per_pixel)
    //             //     .into_par_iter()
    //             //     .map(|_| {
    //             //         let r = self.get_ray(i as f64, j as f64);
    //             //         Self::ray_color(r, &world, self.max_depth)
    //             //     })
    //             //     .reduce(|| Color::new(0.0, 0.0, 0.0), |sum, c| sum + c)
    //             //     / self.samples_per_pixel as f64;
    //             let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    //             for _ in 0..self.samples_per_pixel {
    //                 let r = self.get_ray(i as f64, j as f64);
    //                 pixel_color = pixel_color + Self::ray_color(r, &world, self.max_depth);
    //             }
    //             pixel_color = pixel_color / self.samples_per_pixel as f64;
    //             write_color(pixel_color.to_rgb(), &mut file);
    //             bar.inc(1);
    //         }
    //     }
    //     bar.finish();
    // }

    pub fn render(&mut self, world: &Arc<dyn Hittable>, path: &str) {
        const THREAD_LIMIT: usize = 16;
        const NUM_THREADS: usize = 200;
        self.initialize();
        let img = Arc::new(Mutex::new(ImageBuffer::new(
            self.image_width,
            self.image_height,
        )));
        let bar: ProgressBar = if Self::is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };
        let bar = Arc::new(bar);

        thread::scope(|s| {
            let rows_per_thread = self.image_height / NUM_THREADS as u32;
            let thread_count = Arc::new(AtomicUsize::new(0));
            let thread_number_controller = Arc::new(Condvar::new());

            for thread_id in 0..NUM_THREADS {
                // let world_clone = Arc::clone(&world);
                // let file_clone = Arc::clone(&file);

                let lock_for_condv = Mutex::new(false);
                while !(thread_count.load(Ordering::SeqCst) < THREAD_LIMIT) {
                    thread_number_controller
                        .wait(lock_for_condv.lock().unwrap())
                        .unwrap();
                }

                let mut img_clone = Arc::clone(&img);
                let camera_clone = self.clone();
                let thread_count = Arc::clone(&thread_count);
                let thread_number_controller = Arc::clone(&thread_number_controller);
                let bar = Arc::clone(&bar);
                let start_row = thread_id * rows_per_thread as usize;
                let end_row = if thread_id == NUM_THREADS - 1 {
                    self.image_height as usize
                } else {
                    start_row + rows_per_thread as usize
                };

                thread_count.fetch_add(1, Ordering::SeqCst);
                bar.set_message(format!(
                    "|{} threads outstanding|",
                    thread_count.load(Ordering::SeqCst)
                ));

                s.spawn(move |_| {
                    let mut results: Vec<(usize, usize, [u8; 3])> = Vec::new();

                    for j in start_row..end_row {
                        for i in 0..camera_clone.image_width as usize {
                            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                            for s_i in 0..camera_clone.sqrt_spp {
                                for s_j in 0..camera_clone.sqrt_spp {
                                    let r = camera_clone.get_ray(i as f64, j as f64, s_i, s_j);
                                    pixel_color = pixel_color
                                        + camera_clone.ray_color(r, &world, camera_clone.max_depth);
                                }
                            }
                            pixel_color = pixel_color / camera_clone.samples_per_pixel as f64;
                            // write_color(pixel_color.to_rgb(), &mut file);
                            results.push((i, j, pixel_color.to_rgb()));
                            bar.inc(1);
                        }
                    }

                    // let mut file = file_clone.lock().unwrap();
                    for (i, j, color) in results {
                        write_color(color, &mut img_clone, i, j);
                    }
                    thread_count.fetch_sub(1, Ordering::SeqCst);
                    bar.set_message(format!(
                        "|{} threads outstanding|",
                        thread_count.load(Ordering::SeqCst)
                    ));
                    thread_number_controller.notify_one();
                });
            }
        })
        .unwrap();
        bar.finish();
        let cloned_inner_value = (*img).lock().unwrap().clone();
        let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(cloned_inner_value);
        let mut output_file: File = File::create(path).unwrap();
        match output_image.write_to(&mut output_file, image::ImageOutputFormat::Png) {
            Ok(_) => {}
            Err(_) => println!("Outputting image fails."),
        }
    }
}

pub fn random_double() -> f64 {
    random::<f64>()
}

pub fn random_between(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn random_int(min: i32, max: i32) -> i32 {
    return random_between(min as f64, (max + 1) as f64) as i32;
}
