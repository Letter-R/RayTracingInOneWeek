use super::vec::{Point3, Vec3};
#[derive(Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self.direction() - 2.0 * self.direction().dot(n) * n
    }
}
