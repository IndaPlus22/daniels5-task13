use crate::{material::Material, hittable::{Hittable, HitRecord}, aabb::Aabb, vector::vector::Vec3};

pub struct YzRect {
    mp: Material,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Material) -> Self {
        YzRect { mp: mat, y0: y0, y1: y1, z0: z0, z1: z1, k: k }
    }
}

impl Hittable for YzRect {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: crate::aabb::Aabb) -> Option<crate::aabb::Aabb> {
        return Some(Aabb::new(Vec3::new(self.k-0.0001,self.y0,self.z0), Vec3::new(self.k+0.0001, self.y1, self.z1)));
    }
    fn hit(&self, r: &crate::ray::ray::Ray, t_min: f64, t_may: f64) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - r.origin().x) / r.direction().x;
        if t > t_min && t < t_may {
            let y = r.origin().y + t*r.direction().y;
            let z = r.origin().z + t*r.direction().z;
            if(y > self.y0 && y < self.y1 && z>self.z0 && z < self.z1) {
                let u = (y-self.y0)/(self.y1-self.y0);
                let v = (z-self.z0)/(self.z1-self.z0);
                let outward_normal = Vec3::new(1.0, 0.0, 0.0);
                let mut rec = HitRecord::new(r.at(t), t);
                rec.u = u;
                rec.v = v;
                rec.set_face_normal(*r, outward_normal);
                rec.mat = self.mp;
                return Some(rec);
            }
        }
        return None;
    }
}