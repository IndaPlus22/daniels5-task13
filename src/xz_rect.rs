use crate::{material::Material, hittable::{Hittable, HitRecord}, vector::vector::Vec3};

pub struct XzRect {
    mp: Material,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Material) -> Self {
        XzRect { mp: mat, x0: x0, x1: x1, z0: z0, z1: z1, k: k }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &crate::ray::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - r.origin().y) / r.direction().y;
        if t > t_min && t < t_max {
            let x = r.origin().x + t*r.direction().x;
            let z = r.origin().z + t*r.direction().z;
            if(x > self.x0 && x < self.x1 && z>self.z0 && z < self.z1) {
                let outward_normal = Vec3::new(0.0, 1.0, 0.0);
                let mut rec = HitRecord::new(r.at(t), t);
                rec.set_face_normal(*r, outward_normal);
                rec.mat = self.mp;
                return Some(rec);
            }
        }
        return None;
    }
}