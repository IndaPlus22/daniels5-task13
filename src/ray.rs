pub mod ray {
    use crate::vector::vector::Vec3;
    
    #[derive(Copy, Clone)]
   pub struct Ray {
        Orig: Vec3,
        Dir: Vec3,
        Tm: f64
    }
    impl Ray {
        pub fn new(a: Vec3, b: Vec3, c: f64) -> Ray {
            Ray {
                Orig: a,
                Dir: b,
                Tm: c,
            }
        }
        pub fn origin(&self) -> Vec3 {
            return self.Orig;
        }
        pub fn time(&self) -> f64 {
            return self.Tm;
        }
        pub fn direction(&self) -> Vec3 {
            return self.Dir
        }
        pub fn at(&self, t: f64) -> Vec3 {
            return self.Orig + t*self.Dir;
        }
    }
}