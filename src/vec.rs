use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, Sub, SubAssign},
};

use rand::Rng;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f64 {
        self[0]
    }
    pub fn y(self) -> f64 {
        self[1]
    }
    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }
    // magnitude
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
    //set magnitude to 1
    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }
    // 0,1 to 0,255
    pub fn format_color(self, samples_per_pixel: u64) -> String {
        let ir = (256.0
            * (self[0] / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        let ig = (256.0
            * (self[1] / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        let ib = (256.0
            * (self[2] / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        format!("{} {} {}", ir, ig, ib)
    }

    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            e: [
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
            ],
        }
    }
    //get a random unit vector, otigin (0,0,0)
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }
    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-1.0) * self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

//&Vec3[]
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
///
//&mut Vec3[]
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}
// Vec3=Vec3+Vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}
// Vec3+=Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        };
    }
}
// Vec3=Vec3-Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}
// Vec3-=Vec3
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        };
    }
}
//Vec3=Vec3*f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}
//Vec3*=f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}
//Vec3=f64*Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other[0], self * other[1], self * other[2]],
        }
    }
}
//**tmp:Vec3=Vec3*Vec3
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

//Vec3=Vec3/f64
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other],
        }
    }
}
//Vec3/=f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) -> () {
        *self = Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other],
        };
    }
}

///
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert!((v.x() - 1.0).abs() < f64::EPSILON);
        assert!((v.y() - 2.0).abs() < f64::EPSILON);
        assert!((v.z() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1.dot(v2);
        assert!((result - 32.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1.cross(v2);
        assert!((result.x() - (-3.0)).abs() < f64::EPSILON);
        assert!((result.y() - 6.0).abs() < f64::EPSILON);
        assert!((result.z() - (-3.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert!((v.length() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_normalized() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let result = v.normalized();
        let expected = 1.0 / 3.0;
        assert!((result.x() - expected).abs() < f64::EPSILON);
        assert!((result.y() - (2.0 / 3.0)).abs() < f64::EPSILON);
        assert!((result.z() - (2.0 / 3.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert!((result.x() - 5.0).abs() < f64::EPSILON);
        assert!((result.y() - 7.0).abs() < f64::EPSILON);
        assert!((result.z() - 9.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let diff = v1 - v2;
        assert!((diff.x() - (-3.0)).abs() < f64::EPSILON);
        assert!((diff.y() - (-3.0)).abs() < f64::EPSILON);
        assert!((diff.z() - (-3.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let factor = 2.0;
        let product = v * factor;
        assert!((product.x() - 2.0).abs() < f64::EPSILON);
        assert!((product.y() - 4.0).abs() < f64::EPSILON);
        assert!((product.z() - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let divisor = 2.0;
        let quotient = v / divisor;
        assert!((quotient.x() - 1.0).abs() < f64::EPSILON);
        assert!((quotient.y() - 2.0).abs() < f64::EPSILON);
        assert!((quotient.z() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert!((v1.x() - 5.0).abs() < f64::EPSILON);
        assert!((v1.y() - 7.0).abs() < f64::EPSILON);
        assert!((v1.z() - 9.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_random() {
        let r = 0.0..1.0;
        let v = Vec3::random(r.clone());
        assert!(v.x() >= r.start && v.x() < r.end);
        assert!(v.y() >= r.start && v.y() < r.end);
        assert!(v.z() >= r.start && v.z() < r.end);
    }

    #[test]
    fn test_random_in_unit_sphere() {
        let v = Vec3::random_in_unit_sphere();
        assert!(v.length() < 1.0);
    }

    #[test]
    fn test_reflect() {
        let incident = Vec3::new(1.0, -1.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let reflected = incident.reflect(normal);
        assert!((reflected.x() - 1.0).abs() < f64::EPSILON);
        assert!((reflected.y() - 1.0).abs() < f64::EPSILON);
        assert!((reflected.z() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_refract() {
        let incident = Vec3::new(3.0_f64.sqrt(), -1.0, 0.0).normalized();
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let etai_over_etat = 1.0 / 3.0_f64.sqrt();
        let refracted = incident.refract(normal, etai_over_etat);

        let unit_expect_out = Vec3::new(1.0, -1.0 * (3.0_f64.sqrt()), 0.0).normalized();
        println!("{}", refracted);
        // The refracted vector should have a non-zero x component and a negative y component.
        assert!(
            (refracted.x() - unit_expect_out.x()).abs() < f64::EPSILON,
            "x: Expected {:.15}, got {:.15}",
            unit_expect_out.x(),
            refracted.x()
        );
        assert!(
            (refracted.y() - unit_expect_out.y()).abs() < f64::EPSILON,
            "y: Expected {:.15}, got {:.15}",
            unit_expect_out.y(),
            refracted.y()
        );
        assert!(
            (refracted.z() - unit_expect_out.z()).abs() < f64::EPSILON,
            "z: Expected {:.15}, got {:.15}",
            unit_expect_out.z(),
            refracted.z()
        );
    }
}
