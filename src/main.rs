use std::io::{stderr, Write};
mod hit;
mod ray;
mod sphere;
mod vec;
use hit::{Hit, World};
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};

fn ray_color(r: &Ray, word: &World) -> Color {
    if let Some(rec) = word.hit(r, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
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

    //World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    //Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = //of viewport, so it point to (0,0) of (u,v)
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //photo
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!(
            "\r{:5.2}%", //\r means back to head of line
            (IMAGE_HEIGHT - j) as f64 * 100.0 / IMAGE_HEIGHT as f64
        );
        stderr().flush().unwrap(); //flush empty cache
        for i in 0..IMAGE_WIDTH {
            //先从左到右输出行，再从上到下输出列
            //以左下为（0，0）,红色正比于x,绿色正比于y
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&r, &world);

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("\r  Done!  ");
}
