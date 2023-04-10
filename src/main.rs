use std::{
    io::{stderr, Write},
    sync::{Arc, Mutex},
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
use rayon::prelude::*;
use sphere::Sphere;
use vec::{Color, Point3};

use crate::{
    material::{Dielectric, Lambertian, Metal},
    vec::Vec3,
};
fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    //max depth, set black
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        //hit
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        //no hit, set color

        let unit_direction = r.direction().normalized();
        let t = 0.75 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 1000;
    const MAX_DEPTH: u64 = 20;
    //World
    let r: f64 = (std::f64::consts::PI / 4.0).cos();
    let mut world = World::new();

    let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_left_inner = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, mat_left_inner);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.push(Arc::new(sphere_ground));
    world.push(Arc::new(sphere_center));
    world.push(Arc::new(sphere_left));
    world.push(Arc::new(sphere_left_inner));
    world.push(Arc::new(sphere_right));

    // Camera
    let cam = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
    );

    //photo
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let output_buffer = Arc::new(Mutex::new(vec![
        Color::new(0.0, 0.0, 0.0);
        (IMAGE_WIDTH * IMAGE_HEIGHT) as usize
    ]));

    (0..IMAGE_HEIGHT as usize)
        .collect::<Vec<usize>>()
        .par_iter()
        //.rev()
        .for_each_with(output_buffer.clone(), |output_buffer, &j| {
            eprint!(
                "\r{:5.2}%",
                (IMAGE_HEIGHT as usize - j) as f64 * 100.0 / IMAGE_HEIGHT as f64
            );
            stderr().flush().unwrap();

            let mut line_colors = Vec::with_capacity(IMAGE_WIDTH as usize);

            for i in 0..IMAGE_WIDTH {
                let mut rng = rand::thread_rng();
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }
                line_colors.push(pixel_color);
            }
            output_buffer.lock().unwrap()[(IMAGE_HEIGHT as usize - 1 - j) * IMAGE_WIDTH as usize
                ..(IMAGE_HEIGHT as usize - 1 - j + 1) * IMAGE_WIDTH as usize]
                .copy_from_slice(&line_colors);
        });

    for color in output_buffer.lock().unwrap().iter() {
        println!("{}", color.format_color(SAMPLES_PER_PIXEL));
    }

    eprintln!("\r  Done!  ");
}
