pub mod ray {
    use crate::vector::vector::Vec3;
    
    #[derive(Copy, Clone)]
   pub struct Ray {
        Orig: Vec3,
        Dir: Vec3,
    }
    impl Ray {
        pub fn new(a: Vec3, b: Vec3) -> Ray {
            Ray {
                Orig: a,
                Dir: b,
            }
        }
        pub fn origin(&self) -> Vec3 {
            return self.Orig;
        }
        pub fn direction(&self) -> Vec3 {
            return self.Dir
        }
        pub fn at(&self, t: f64) -> Vec3 {
            return self.Orig + t*self.Dir;
        }
    }
}