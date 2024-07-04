
use std::{f64::INFINITY, fs::File};
mod util;
use util::*;
#[macro_use]
extern crate lazy_static;

const AUTHOR: &str = "CHENG";

fn main() {
    let path = "output/material_fuzz.ppm";
    
    lazy_static!{
        static ref MATERIAL_GROUND: Lambertian = Lambertian::new(Color::new(0.8, 0.8, 0.0));
        static ref MATERIAL_CENTER: Lambertian = Lambertian::new(Color::new(0.1, 0.2, 0.5));
        static ref MATERIAL_LEFT: Metal = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
        static ref MATERIAL_RIGHT: Metal = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);
    }
    

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    // Objects
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Some(&*MATERIAL_CENTER))));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Some(&*MATERIAL_GROUND))));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Some(&*MATERIAL_LEFT))));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Some(&*MATERIAL_RIGHT))));
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
