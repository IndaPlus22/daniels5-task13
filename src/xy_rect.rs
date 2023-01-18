use crate::{material::Material, hittable::{Hittable, HitRecord}, aabb::Aabb, vector::vector::Vec3};

pub struct XyRect {
    mp: Material,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Material) -> Self {
        XyRect { mp: mat, x0: x0, x1: x1, y0: y0, y1: y1, k: k }
    }
}

impl Hittable for XyRect {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: crate::aabb::Aabb) -> Option<crate::aabb::Aabb> {
        return Some(Aabb::new(Vec3::new(self.x0,self.y0,self.k-0.0001), Vec3::new(self.x1, self.y1, self.k+0.0001)));
    }
    fn hit(&self, r: &crate::ray::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - r.origin().z) / r.direction().z;
        if t > t_min && t < t_max {
            let x = r.origin().x + t*r.direction().x;
            let y = r.origin().y + t*r.direction().y;
            if(x > self.x0 && x < self.x1 && y>self.y0 && y < self.y1) {
                let u = (x-self.x0)/(self.x1-self.x0);
                let v = (y-self.y0)/(self.y1-self.y0);
                let outward_normal = Vec3::new(0.0, 0.0, 1.0);
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