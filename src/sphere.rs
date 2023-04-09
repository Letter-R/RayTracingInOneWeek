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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length().powi(2);
        let half_b = ray.direction().dot(oc);
        let c = oc.length().powi(2) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        //2 root
        //2正，取符合范围小的
        //1正1负，在内部，取正的
        //2负，在背后，忽略

        let sqrtd = discriminant.sqrt();
        //较小的
        let root = (-half_b - sqrtd) / a;
        if root < t_max && root > t_min {
            let mut rec = HitRecord {
                p: ray.at(root),
                normal: Vec3::new(0.0, 0.0, 0.0),
                t: root,
                front_face: false,
                material: self.material.clone(),
            };
            rec.set_face_normal(ray, (rec.p - self.center).normalized());
            return Some(rec);
        }
        //较大的
        let root = (-half_b + sqrtd) / a;
        if root < t_max && root > t_min {
            let mut rec = HitRecord {
                p: ray.at(root),
                normal: Vec3::new(0.0, 0.0, 0.0),
                t: root,
                front_face: false,
                material: self.material.clone(),
            };
            rec.set_face_normal(ray, (rec.p - self.center).normalized());
            return Some(rec);
        }
        //0 root，不相交
        //1 root，相切，忽略

        None

        //
    }
}
