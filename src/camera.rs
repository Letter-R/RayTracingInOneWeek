use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
}

impl Camera {
    /// vfov：垂直视场角
    /// aspect_ratio：长宽比
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        const FOCAL_LENGTH: f64 = 1.0;

        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalized();
        let cu = vup.cross(cw).normalized();
        let cv = cw.cross(cu);

        let h = viewport_width * cu;
        let v = viewport_height * cv;

        let llc = lookfrom - h / 2.0 - v / 2.0 - cw;

        Camera {
            origin: lookfrom,
            lower_left_corner: llc,
            horizontal: h,
            vertical: v,
        }
    }

    //input s,t in 0 to 1, output a ray
    //s,t of viewport
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
