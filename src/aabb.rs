use crate::{vector::vector::Vec3, ray::ray::Ray};
#[derive(Clone, Copy)]
pub struct Aabb {
    minimum: Vec3,
    maximum: Vec3,
}

impl Aabb {
    pub fn new(a: Vec3, b: Vec3) -> Aabb {
        Aabb { minimum: a, maximum: b }
    }
    pub fn min(&self) -> Vec3 {
        return self.minimum
    }
    pub fn max(&self) -> Vec3 {
        return self.maximum;
    }
    pub fn hit(&self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction().getByNumber(a);
            let mut t0 = (self.minimum.getByNumber(a) - r.origin().getByNumber(a)) * inv_d;
            let mut t1 = (self.maximum.getByNumber(a) - r.origin().getByNumber(a)) * inv_d;
            if(inv_d < 0.0) {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }
            if (t0 > t_min) {
                t_min = t0;
            }
            if (t1 < t_max) {
                t_max = t1;
            }
            if (t_max <= t_min) {
                return false;
            }
        }
        return true;
    }
    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb{
        let small = Vec3::new(f64::min(box0.minimum.x, box1.minimum.x), f64::min(box0.minimum.y, box1.minimum.y),f64::min(box0.minimum.z, box1.minimum.z));
        let big = Vec3::new(f64::min(box0.maximum.x, box1.maximum.x), f64::min(box0.maximum.y, box1.maximum.y),f64::min(box0.maximum.z, box1.maximum.z));
        return Aabb::new(small, big);

    }
}