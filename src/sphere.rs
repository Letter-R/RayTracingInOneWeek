use std::rc::Rc;

use crate::material::Scatter;

use super::hit::{Hit, HitRecord};
use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Scatter>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Rc<dyn Scatter>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            material: m,
        }
    }
}

//(P−C)⋅(P−C)=r2
//P(t)=A+tb, A is ray source
//t^2 b⋅b+t 2b⋅(A−C)+(A−C)⋅(A−C)−r^2=0
impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = r.direction().dot(oc);
        let c = oc.length().powi(2) - self.radius.powi(2);
        let discriminant = half_b * half_b - a * c;
        //0 root
        if discriminant < 0.0 {
            return None;
        }
        // out of sphere
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        //
        let mut rec = HitRecord {
            p: r.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: root,
            front_face: false,
            material: self.material.clone(),
        };
        rec.set_face_normal(r, (rec.p - self.center).normalized());
        Some(rec)
    }
}
