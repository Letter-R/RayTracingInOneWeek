use std::{
    io::{stderr, Write},
    rc::Rc,
};
mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec;
use camera::Camera;
use hit::{Hit, World};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};

use crate::material::{Lambertian, Metal};
fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    //max depth, set black
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        //hit
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            0.25 * attenuation + 0.75 * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        //no hit, set color

        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 10;
    const MAX_DEPTH: u64 = 5;
    //World
    let mut world = World::new();
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.8));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    //Camera
    let camera = Camera::new();

    //photo
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!(
            "\r{:5.2}%", //\r means back to head of line
            (IMAGE_HEIGHT - j) as f64 * 100.0 / IMAGE_HEIGHT as f64
        );
        stderr().flush().unwrap(); //flush empty cache
        for i in 0..IMAGE_WIDTH {
            // for each pixel, do this:
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen(); //[0,1)
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("\r  Done!  ");
}
