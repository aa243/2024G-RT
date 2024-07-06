
use std::{f64::INFINITY, fs::File, f64::consts::PI};
mod util;
use util::*;
#[macro_use]
extern crate lazy_static;

const AUTHOR: &str = "CHENG";

fn main() {
    let path = "output/final_scene.ppm";
    let R = (PI / 4.0).cos();
    let mut world = HittableList::new();

    lazy_static! {
        static ref GROUND_MATERIAL: Lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
        static ref MATERIAL1: Dielectric = Dielectric::new(1.5);
        static ref MATERIAL2: Lambertian = Lambertian::new(Color::new(0.4, 0.2, 0.1));
        static ref MATERIAL3: Metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    }
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Some(&*GROUND_MATERIAL))));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Some(&*MATERIAL1))));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Some(&*MATERIAL2))));
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Some(&*MATERIAL3))));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0,0.2,0.0)).length() > 0.9{
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random().element_mul(Color::random());
                    let sphere_material = Lambertian::new(albedo);
                    let static_material: &'static Lambertian = Box::leak(Box::new(sphere_material));
                    world.add(Box::new(Sphere::new(center, 0.2, Some(static_material))));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_between(0.5,1.0);
                    let fuzz = random_between(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    let static_material: &'static Metal = Box::leak(Box::new(sphere_material));
                    world.add(Box::new(Sphere::new(center, 0.2, Some(static_material))));
                }
                else {
                    //glass
                    let sphere_material = Dielectric::new(1.5);
                    let static_material: &'static Dielectric = Box::leak(Box::new(sphere_material));
                    world.add(Box::new(Sphere::new(center, 0.2, Some(static_material))));
                }
            }
        }
    }
    
    let material = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let static_material: &'static Lambertian = Box::leak(Box::new(material));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist);

    let boxed_world = Box::new(world) as Box<dyn Hittable>;

    cam.render(&boxed_world, path);

    // Save the image
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    // let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    // let mut output_file: File = File::create(path).unwrap();
    // match output_image.write_to(&mut output_file, image::ImageOutputFormat::Png) {
    //     Ok(_) => {}
    //     Err(_) => println!("Outputting image fails."),
    // }
}
