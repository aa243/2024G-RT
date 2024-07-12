
use std::{f64::INFINITY, fs::File, f64::consts::PI};
mod util;
use util::*;
use std::sync::Arc;
#[macro_use]
extern crate lazy_static;

const AUTHOR: &str = "CHENG";

fn bouncing_spheres() {
    let path = "output/book2/bouncing_sphere_with_background_color.png";
    let R = (PI / 4.0).cos();
    let mut world = HittableList::new();
    let checker: Arc<dyn Texture> = Arc::new(Checker_Texture::new_by_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9), 0.32));
    let ground_material = Lambertian::new(checker);
    let static_material: &'static Lambertian = Box::leak(Box::new(ground_material));

    lazy_static! {
        // static ref CHECKER: Arc<dyn Texture> = Arc::new(Checker_Texture::new_by_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9), 10.0));
        // static ref GROUND_MATERIAL: Lambertian = Lambertian::new(checker);
        static ref MATERIAL1: Dielectric = Dielectric::new(1.5);
        static ref MATERIAL2: Lambertian = Lambertian::new_by_color(Color::new(0.4, 0.2, 0.1));
        static ref MATERIAL3: Metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    }
    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, Some(static_material))));
    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, 1.0, 0.0), 1.0, Some(&*MATERIAL1))));
    world.add(Arc::new(Sphere::new_static(Point3::new(-4.0, 1.0, 0.0), 1.0, Some(&*MATERIAL2))));
    world.add(Arc::new(Sphere::new_static(Point3::new(4.0, 1.0, 0.0), 1.0, Some(&*MATERIAL3))));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0,0.2,0.0)).length() > 0.9{
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random().element_mul(Color::random());
                    let sphere_material = Lambertian::new_by_color(albedo);
                    let static_material: &'static Lambertian = Box::leak(Box::new(sphere_material));
                    let center2 = center + Vec3::new(0.0, random_between(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new(center, 0.2, Some(static_material), center2)));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_between(0.5,1.0);
                    let fuzz = random_between(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    let static_material: &'static Metal = Box::leak(Box::new(sphere_material));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, Some(static_material))));
                }
                else {
                    //glass
                    let sphere_material = Dielectric::new(1.5);
                    let static_material: &'static Dielectric = Box::leak(Box::new(sphere_material));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, Some(static_material))));
                }
            }
        }
    }

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let background = Color::new(0.7, 0.8, 1.0);
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist, background);
    let mut bvh_world: HittableList = HittableList::new();
    bvh_world.add(Arc::new(BvhNode::new_by_object_list(&world)));
    let boxed_world = Arc::new(bvh_world) as Arc<dyn Hittable>;

    cam.render(&boxed_world, path);

    // Save the image
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
}

fn checkered_spheres() {
    let path = "output/book2/checkered_spheres.png";
    let R = (PI / 4.0).cos();
    let mut world = HittableList::new();

    let checker: Arc<dyn Texture> = Arc::new(Checker_Texture::new_by_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9), 0.32));
    let ground_material = Lambertian::new(checker);
    let static_material: &'static Lambertian = Box::leak(Box::new(ground_material));

    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, -10.0, 0.0), 10.0, Some(static_material))));
    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, 10.0, 0.0), 10.0, Some(static_material))));
    // world.add(Arc::new(Sphere::new_static(Point3::new(0.0, 1.0, 0.0), 1.0, Some(&*MATERIAL1))));
    // world.add(Arc::new(Sphere::new_static(Point3::new(-4.0, 1.0, 0.0), 1.0, Some(&*MATERIAL2))));
    // world.add(Arc::new(Sphere::new_static(Point3::new(4.0, 1.0, 0.0), 1.0, Some(&*MATERIAL3))));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.7, 0.8, 1.0);
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist, background);
    let mut bvh_world: HittableList = HittableList::new();
    bvh_world.add(Arc::new(BvhNode::new_by_object_list(&world)));
    let boxed_world = Arc::new(bvh_world) as Arc<dyn Hittable>;

    cam.render(&boxed_world, path);

    // Save the image
    println!("Output image as \"{}\"\n Author: {}", path, AUTHOR);
}

fn earth() {
    let path = "output/book2/earth.png";
    let earth_texture = Arc::new(Image_Texture::new("support/earthmap.jpg"));
    let earth_surface = Lambertian::new(earth_texture);
    let static_material: &'static Lambertian = Box::leak(Box::new(earth_surface));
    let globe = Arc::new(Sphere::new_static(Point3::new(0.0, 0.0, 0.0), 2.0, Some(static_material)));

    let mut world = HittableList::new();
    world.add(globe);

    let mut bvh_world: HittableList = HittableList::new();
    bvh_world.add(Arc::new(BvhNode::new_by_object_list(&world)));
    let boxed_world = Arc::new(bvh_world) as Arc<dyn Hittable>;

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new(0.0,0.0,12.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.7, 0.8, 1.0);
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist, background);
    cam.render(&boxed_world, path);

    // Save the image
    println!("Output image as \"{}\"\n Author: {}", path, AUTHOR);
}

fn perlin_spheres() {
    let path = "output/book2/perlin_noise_marbled_texture.png";
    let perlin_texture = Arc::new(Noise_Texture::new(4.0));
    let earth_surface = Lambertian::new(perlin_texture);
    let static_material: &'static Lambertian = Box::leak(Box::new(earth_surface));
    let globe1 = Arc::new(Sphere::new_static(Point3::new(0.0, 2.0, 0.0), 2.0, Some(static_material)));
    let globe2 = Arc::new(Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, Some(static_material)));

    let mut world = HittableList::new();
    world.add(globe1);
    world.add(globe2);

    let mut bvh_world: HittableList = HittableList::new();
    bvh_world.add(Arc::new(BvhNode::new_by_object_list(&world)));
    let boxed_world = Arc::new(bvh_world) as Arc<dyn Hittable>;

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new(13.0,2.0,3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.7, 0.8, 1.0);
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist, background);

    cam.render(&boxed_world, path);

    // Save the image
    println!("Output image as \"{}\"\n Author: {}", path, AUTHOR);
}

fn quads() {
    let path = "output/book2/quads.png";
    
    lazy_static! {
        static ref LEFT_RED: Lambertian = Lambertian::new_by_color(Color::new(1.0,0.2,0.2));
        static ref BACK_GREEN: Lambertian = Lambertian::new_by_color(Color::new(0.2,1.0,0.2));
        static ref RIGHT_BLUE: Lambertian = Lambertian::new_by_color(Color::new(0.2,0.2,1.0));
        static ref UPPER_ORANGE: Lambertian = Lambertian::new_by_color(Color::new(1.0,0.5,0.0));
        static ref LOWER_TEAL: Lambertian = Lambertian::new_by_color(Color::new(0.2,0.8,0.8));
    }

    let globe1 = Arc::new(Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), Some(&*LEFT_RED)));
    let globe2 = Arc::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), Some(&*BACK_GREEN)));
    let globe3 = Arc::new(Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), Some(&*RIGHT_BLUE)));
    let globe4 = Arc::new(Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), Some(&*UPPER_ORANGE)));
    let globe5 = Arc::new(Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), Some(&*LOWER_TEAL)));

    let mut world = HittableList::new();
    world.add(globe1);
    world.add(globe2);
    world.add(globe3);
    world.add(globe4);
    world.add(globe5);

    let mut bvh_world: HittableList = HittableList::new();
    bvh_world.add(Arc::new(BvhNode::new_by_object_list(&world)));
    let boxed_world = Arc::new(bvh_world) as Arc<dyn Hittable>;

    let aspect_ratio = 1.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 80.0;
    let lookfrom = Point3::new(0.0,0.0,9.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.7, 0.8, 1.0);
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist, background);

    cam.render(&boxed_world, path);

    // Save the image
    println!("Output image as \"{}\"\n Author: {}", path, AUTHOR);
}

fn simple_light() {
    let path = "output/book2/lights_with_sphere.png";
    
    lazy_static! {
        static ref DIFF_LIGHT: DiffuseLight = DiffuseLight::new_by_color(Color::new(4.0,4.0,4.0));
    }

    let pertext = Arc::new(Noise_Texture::new(4.0));
    let marble = Lambertian::new(pertext);
    let marble: &'static Lambertian = Box::leak(Box::new(marble));

    let globe1 = Arc::new(Sphere::new_static(Point3::new(0.0,-1000.0,0.0), 1000.0, Some(marble)));
    let globe2 = Arc::new(Sphere::new_static(Point3::new(0.0,2.0,0.0), 2.0, Some(marble))); 
    let globe3 = Arc::new(Sphere::new_static(Point3::new(0.0,7.0,0.0), 2.0, Some(&*DIFF_LIGHT)));
    let quad1 = Arc::new(Quad::new(Point3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Some(&*DIFF_LIGHT))); 
    
    let mut world = HittableList::new();
    world.add(globe1);
    world.add(globe2);
    world.add(globe3);
    world.add(quad1);

    let mut bvh_world: HittableList = HittableList::new();
    bvh_world.add(Arc::new(BvhNode::new_by_object_list(&world)));
    let boxed_world = Arc::new(bvh_world) as Arc<dyn Hittable>;

    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Point3::new(26.0,3.0,6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist, background);

    cam.render(&boxed_world, path);

    // Save the image
    println!("Output image as \"{}\"\n Author: {}", path, AUTHOR);
}

fn cornell_box() {
    let path = "output/book2/cornell_box.png";
    
    lazy_static! {
        static ref LIGHT: DiffuseLight = DiffuseLight::new_by_color(Color::new(15.0,15.0,15.0));
        static ref RED: Lambertian = Lambertian::new_by_color(Color::new(0.65,0.05,0.05));
        static ref WHITE: Lambertian = Lambertian::new_by_color(Color::new(0.73,0.73,0.73));
        static ref GREEN: Lambertian = Lambertian::new_by_color(Color::new(0.12,0.45,0.15));
    }

    let quad1 = Arc::new(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(&*GREEN)));
    let quad2 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(&*RED)));
    let quad3 = Arc::new(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), Some(&*LIGHT)));
    let quad4 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(&*WHITE)));
    let quad5 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Some(&*WHITE)));
    let quad6 = Arc::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), Some(&*WHITE)));
    let box1 = get_box(Point3::new(130.0,0.0,65.0), Point3::new(295.0, 165.0, 230.0), Some(&*WHITE));
    let box2 = get_box(Point3::new(265.0,0.0,295.0), Point3::new(430.0, 330.0, 460.0), Some(&*WHITE));
    
    let mut world = HittableList::new();
    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    world.add(quad5);
    world.add(quad6);
    world.add(box1);
    world.add(box2);

    let mut bvh_world: HittableList = HittableList::new();
    bvh_world.add(Arc::new(BvhNode::new_by_object_list(&world)));
    let boxed_world = Arc::new(bvh_world) as Arc<dyn Hittable>;

    let aspect_ratio = 1.0;
    let image_width = 400;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let vfov = 40.0;
    let lookfrom = Point3::new(278.0,278.0,-800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);
    let mut cam = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist, background);

    cam.render(&boxed_world, path);

    // Save the image
    println!("Output image as \"{}\"\n Author: {}", path, AUTHOR);
}

fn main() {
    
    match 7 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => println!("Invalid input."),
    }

    // let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    // let mut output_file: File = File::create(path).unwrap();
    // match output_image.write_to(&mut output_file, image::ImageOutputFormat::Png) {
    //     Ok(_) => {}
    //     Err(_) => println!("Outputting image fails."),
    // }
}
