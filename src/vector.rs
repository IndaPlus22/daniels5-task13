pub mod vector {
    use std::{ops};

    #[derive(Copy, Clone)]
    pub struct Vec3{
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub r: f64,
        pub g: f64,
        pub b: f64
    }

    impl Vec3 {
        pub fn new(x: f64, y: f64, z: f64) -> Vec3{
            Vec3 { x: (x), y: (y), z: (z) , r: x, g: y, b: z}
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
}