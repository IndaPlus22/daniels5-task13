use rand::Rng;
pub mod vector {
    use std::{ops, cmp, iter::Sum};

    use rand::Rng;

    use crate::random::random_double;

    #[derive(Copy, Clone)]
    pub struct Vec3{
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Vec3 {
        pub fn new(x: f64, y: f64, z: f64) -> Vec3{
            Vec3 { x: (x), y: (y), z: (z)}
        }
        pub fn length(&self) -> f64 {
            return f64::sqrt(self.x*self.x + self.y*self.y + self.z*self.z);
        }
        pub fn squared_length(&self) -> f64 {
            return self.x*self.x + self.y*self.y + self.z*self.z;
        }
        pub fn dot(v1:Vec3,v2: Vec3) -> f64 {
            return v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
        }
        pub fn cross(v1:Vec3, v2:Vec3) -> Vec3 {
            Vec3::new((v1.y*v2.z - v1.z*v2.y), -(v1.x*v2.z - v1.z*v2.x), (v1.x*v2.y-v1.y*v2.x))
        }
        pub fn unit_vector(v1: Vec3) -> Vec3{
            return v1 / v1.length();
        }
        pub fn random(min: f64, max: f64) -> Vec3 {
            return Vec3::new(random_double(min, max), random_double(min, max), random_double(min, max))
        }

        pub fn random_in_unit_sphere() -> Vec3{
            while true {
                let p = Vec3::random(-1.0, 1.0);
                if (p.squared_length() < 1.0) {
                    return p;
                }
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }
        pub fn random_unit_vector() -> Vec3 {
            return Vec3::unit_vector(Vec3::random_in_unit_sphere())
        }
        pub fn near_zero(&self) -> bool {
            let s = 1e-8;
            return (self.x.abs() < s && self.y.abs() < s && self.z.abs() < s);
        }
        //Calcukate refraction using Snell's Law, 
        pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
            let cos_theta = f64::min(Vec3::dot(-1.0*uv, n), 1.0);
            let r_out_perp = etai_over_etat * (uv + cos_theta*n);
            let r_out_parallel = -1.0*((1.0 - r_out_perp.squared_length()).abs()).sqrt() * n;
            return r_out_perp + r_out_parallel;
        }
        pub fn random_in_unit_disk() -> Vec3 {
            while true {
                let p = Vec3::new(random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0);
                if (p.squared_length() < 1.0) {
                    return p;
                }
            }
            return Vec3::new(100.0, 1000.0,1000.0);
        }
        pub fn getByNumber(&self, index: i32) -> f64 {
            if(index == 0) {
                return self.x;
            } else if (index == 1) {
                return self.y;
            } else {
                return self.z;
            }
        }

    }
    impl ops::Add<Vec3> for Vec3 {
        type Output = Vec3;

        fn add(self, _rhs: Vec3) -> Self::Output {
            Vec3::new(self.x+_rhs.x, self.y+_rhs.y, self.z+_rhs.z)
        }
    }
    impl ops::Sub<Vec3> for Vec3 {
        type Output = Vec3;

        fn sub(self, _rhs: Vec3) -> Self::Output {
            Vec3::new(self.x-_rhs.x, self.y-_rhs.y, self.z-_rhs.z)
        }
    }
    impl ops::Mul<Vec3> for Vec3 {
        type Output = Vec3;

        fn mul(self, _rhs: Vec3) -> Self::Output {
            Vec3::new(self.x*_rhs.x, self.y*_rhs.y, self.z*_rhs.z)
        }
    }
    impl ops::Mul<Vec3> for f64 {
        type Output = Vec3;
        fn mul(self, rhs: Vec3) -> Self::Output {
            Vec3::new(rhs.x*self, rhs.y*self, rhs.z*self)
        }
    }
    impl ops::Div<Vec3> for Vec3 {
        type Output = Vec3;
        fn div(self, _rhs: Vec3) -> Self::Output {
            Vec3::new(self.x / _rhs.x, self.y / _rhs.y, self.z / _rhs.z)
        }
    }
    impl ops::Mul<f64> for Vec3 {
        type Output = Vec3;
        fn mul(self, rhs: f64) -> Self::Output {
            Vec3::new(self.x*rhs, self.y*rhs, self.y*rhs)
        }
    }
    impl ops::Div<f64> for Vec3 {
        type Output = Vec3;
        fn div(self, rhs: f64) -> Self::Output {
            Vec3::new(self.x/rhs, self.y/rhs, self.z/rhs)
        }
    }
    impl ops::AddAssign<Vec3> for Vec3 {
        fn add_assign(&mut self, rhs: Vec3) {
            self.x+=rhs.x;
            self.y+=rhs.y;
            self.z+=rhs.z;
        }
    }
    impl ops::MulAssign<Vec3> for Vec3 {
        fn mul_assign(&mut self, rhs: Vec3) {
            self.x *= rhs.x;
            self.y *= rhs.y;
            self.z *= rhs.z;
        }
    }
    impl ops::DivAssign<Vec3> for Vec3 {
        fn div_assign(&mut self, rhs: Vec3) {
            self.x /= rhs.x;
            self.y /= rhs.y;
            self.z /= rhs.z;
        }
    }
    impl ops::SubAssign<Vec3> for Vec3 {
        fn sub_assign(&mut self, rhs: Vec3) {
            self.x -= rhs.x;
            self.y -= rhs.y;
            self.z -= rhs.z;
        }
    }
    impl ops::MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, rhs: f64) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }
    impl ops::DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, rhs: f64) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
        }
    }
    impl Sum<Self> for Vec3 {
        fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item = Self>,
            {
                iter.fold(Self {x: 0.0, y: 0.0, z: 0.0}, |a, b| Self {
                    x: a.x + b.x,
                    y: a.y + b.y,
                    z: a.z + b.z
                })
            }
    }
}