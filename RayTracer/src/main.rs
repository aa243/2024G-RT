
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
    let ground_material = Arc::new(Lambertian::new(checker));

    let MATERIAL1 = Arc::new(Dielectric::new(1.5));
    let MATERIAL2 = Arc::new(Lambertian::new_by_color(Color::new(0.4, 0.2, 0.1)));
    let MATERIAL3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, Some(ground_material.clone() as Arc<dyn Material>))));
    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, 1.0, 0.0), 1.0, Some(MATERIAL1.clone() as Arc<dyn Material>))));
    world.add(Arc::new(Sphere::new_static(Point3::new(-4.0, 1.0, 0.0), 1.0, Some(MATERIAL2.clone() as Arc<dyn Material>))));
    world.add(Arc::new(Sphere::new_static(Point3::new(4.0, 1.0, 0.0), 1.0, Some(MATERIAL3.clone() as Arc<dyn Material>))));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0,0.2,0.0)).length() > 0.9{
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random().element_mul(Color::random());
                    let sphere_material = Arc::new(Lambertian::new_by_color(albedo));
                    let center2 = center + Vec3::new(0.0, random_between(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new(center, 0.2, Some(sphere_material.clone() as Arc<dyn Material>), center2)));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_between(0.5,1.0);
                    let fuzz = random_between(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, Some(sphere_material.clone() as Arc<dyn Material>))));
                }
                else {
                    //glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, Some(sphere_material.clone() as Arc<dyn Material>))));
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
    let ground_material = Arc::new(Lambertian::new(checker));

    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, -10.0, 0.0), 10.0, Some(ground_material.clone() as Arc<dyn Material>))));
    world.add(Arc::new(Sphere::new_static(Point3::new(0.0, 10.0, 0.0), 10.0, Some(ground_material.clone() as Arc<dyn Material>))));
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
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new_static(Point3::new(0.0, 0.0, 0.0), 2.0, Some(earth_surface.clone() as Arc<dyn Material>)));

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
    let earth_surface = Arc::new(Lambertian::new(perlin_texture));
    let globe1 = Arc::new(Sphere::new_static(Point3::new(0.0, 2.0, 0.0), 2.0, Some(earth_surface.clone() as Arc<dyn Material>)));
    let globe2 = Arc::new(Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, Some(earth_surface.clone() as Arc<dyn Material>)));

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
    
    let LEFT_RED = Arc::new(Lambertian::new_by_color(Color::new(1.0,0.2,0.2)));
    let BACK_GREEN = Arc::new(Lambertian::new_by_color(Color::new(0.2,1.0,0.2)));
    let RIGHT_BLUE = Arc::new(Lambertian::new_by_color(Color::new(0.2,0.2,1.0)));
    let UPPER_ORANGE = Arc::new(Lambertian::new_by_color(Color::new(1.0,0.5,0.0)));
    let LOWER_TEAL = Arc::new(Lambertian::new_by_color(Color::new(0.2,0.8,0.8)));

    let globe1 = Arc::new(Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), Some(LEFT_RED)));
    let globe2 = Arc::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), Some(BACK_GREEN)));
    let globe3 = Arc::new(Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), Some(RIGHT_BLUE)));
    let globe4 = Arc::new(Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), Some(UPPER_ORANGE)));
    let globe5 = Arc::new(Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), Some(LOWER_TEAL)));

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
    
    let DIFF_LIGHT = Arc::new(DiffuseLight::new_by_color(Color::new(4.0,4.0,4.0)));

    let pertext = Arc::new(Noise_Texture::new(4.0));
    let marble = Arc::new(Lambertian::new(pertext));

    let globe1 = Arc::new(Sphere::new_static(Point3::new(0.0,-1000.0,0.0), 1000.0, Some(marble.clone() as Arc<dyn Material>)));
    let globe2 = Arc::new(Sphere::new_static(Point3::new(0.0,2.0,0.0), 2.0, Some(marble.clone() as Arc<dyn Material>))); 
    let globe3 = Arc::new(Sphere::new_static(Point3::new(0.0,7.0,0.0), 2.0, Some(DIFF_LIGHT.clone() as Arc<dyn Material>)));
    let quad1 = Arc::new(Quad::new(Point3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Some(DIFF_LIGHT.clone() as Arc<dyn Material>))); 
    
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

    let LIGHT = Arc::new(DiffuseLight::new_by_color(Color::new(15.0,15.0,15.0)));
    let RED = Arc::new(Lambertian::new_by_color(Color::new(0.65,0.05,0.05)));
    let WHITE = Arc::new(Lambertian::new_by_color(Color::new(0.73,0.73,0.73)));
    let GREEN = Arc::new(Lambertian::new_by_color(Color::new(0.12,0.45,0.15)));

    let quad1 = Arc::new(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(Arc::clone(&GREEN) as Arc<dyn Material>)));
    let quad2 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(Arc::clone(&RED) as Arc<dyn Material>)));
    let quad3 = Arc::new(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), Some(Arc::clone(&LIGHT) as Arc<dyn Material>)));
    let quad4 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>)));
    let quad5 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>)));
    let quad6 = Arc::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>)));
    let box1 = get_box(Point3::new(0.0,0.0,0.0), Point3::new(165.0, 330.0, 165.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    let box2 = get_box(Point3::new(0.0,0.0,0.0), Point3::new(165.0, 165.0, 165.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0,0.0,65.0)));
    
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

fn cornell_smoke() {
    let path = "output/book2/cornell_smoke.png";

    let LIGHT = Arc::new(DiffuseLight::new_by_color(Color::new(7.0,7.0,7.0)));
    let RED = Arc::new(Lambertian::new_by_color(Color::new(0.65,0.05,0.05)));
    let WHITE = Arc::new(Lambertian::new_by_color(Color::new(0.73,0.73,0.73)));
    let GREEN = Arc::new(Lambertian::new_by_color(Color::new(0.12,0.45,0.15)));

    let quad1 = Arc::new(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(Arc::clone(&GREEN) as Arc<dyn Material>)));
    let quad2 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(Arc::clone(&RED) as Arc<dyn Material>)));
    let quad3 = Arc::new(Quad::new(Point3::new(113.0, 554.0, 127.0), Vec3::new(330.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 305.0), Some(Arc::clone(&LIGHT) as Arc<dyn Material>)));
    let quad4 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>)));
    let quad5 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>)));
    let quad6 = Arc::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>)));
    let box1 = get_box(Point3::new(0.0,0.0,0.0), Point3::new(165.0, 330.0, 165.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    let box2 = get_box(Point3::new(0.0,0.0,0.0), Point3::new(165.0, 165.0, 165.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0,0.0,65.0)));
    let smoke1 = Arc::new(ConstantMedium::new_by_color(Arc::clone(&box1) as Arc<dyn Hittable>, 0.01, Color::new(0.0,0.0,0.0)));
    let smoke2 = Arc::new(ConstantMedium::new_by_color(Arc::clone(&box2) as Arc<dyn Hittable>, 0.01, Color::new(1.0,1.0,1.0)));
    
    let mut world = HittableList::new();
    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    world.add(quad5);
    world.add(quad6);
    world.add(smoke1);
    world.add(smoke2);

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

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let path = "output/book2/final_scene.png";

    let LIGHT = Arc::new(DiffuseLight::new_by_color(Color::new(7.0,7.0,7.0)));
    let RED = Arc::new(Lambertian::new_by_color(Color::new(0.65,0.05,0.05)));
    let WHITE = Arc::new(Lambertian::new_by_color(Color::new(0.73,0.73,0.73)));
    let GREEN = Arc::new(Lambertian::new_by_color(Color::new(0.12,0.45,0.15)));

    let quad1 = Arc::new(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(Arc::clone(&GREEN) as Arc<dyn Material>)));
    let quad2 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(Arc::clone(&RED) as Arc<dyn Material>)));
    let quad3 = Arc::new(Quad::new(Point3::new(113.0, 554.0, 127.0), Vec3::new(330.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 305.0), Some(Arc::clone(&LIGHT) as Arc<dyn Material>)));
    let quad4 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>)));
    let quad5 = Arc::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>)));
    let quad6 = Arc::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>)));
    let box1 = get_box(Point3::new(0.0,0.0,0.0), Point3::new(165.0, 330.0, 165.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    let box2 = get_box(Point3::new(0.0,0.0,0.0), Point3::new(165.0, 165.0, 165.0), Some(Arc::clone(&WHITE) as Arc<dyn Material>));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0,0.0,65.0)));
    let smoke1 = Arc::new(ConstantMedium::new_by_color(Arc::clone(&box1) as Arc<dyn Hittable>, 0.01, Color::new(0.0,0.0,0.0)));
    let smoke2 = Arc::new(ConstantMedium::new_by_color(Arc::clone(&box2) as Arc<dyn Hittable>, 0.01, Color::new(1.0,1.0,1.0)));
    
    let mut world = HittableList::new();
    world.add(quad1);
    world.add(quad2);
    world.add(quad3);
    world.add(quad4);
    world.add(quad5);
    world.add(quad6);
    world.add(smoke1);
    world.add(smoke2);

    let mut bvh_world: HittableList = HittableList::new();
    bvh_world.add(Arc::new(BvhNode::new_by_object_list(&world)));
    let boxed_world = Arc::new(bvh_world) as Arc<dyn Hittable>;

    let aspect_ratio = 1.0;
    let image_width = image_width;
    let samples_per_pixel = samples_per_pixel;
    let max_depth = max_depth;
    let vfov = 40.0;
    let lookfrom = Point3::new(478.0,278.0,-600.0);
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
    
    match 9 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800,10000,40),
        _ => final_scene(400,250,4),
    }

    // let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    // let mut output_file: File = File::create(path).unwrap();
    // match output_image.write_to(&mut output_file, image::ImageOutputFormat::Png) {
    //     Ok(_) => {}
    //     Err(_) => println!("Outputting image fails."),
    // }
}
