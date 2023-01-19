use crate::{material::Material, hittable::{Hittable, HitRecord}, vector::vector::Vec3};

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
//Hittable function bassicly first checks if the ray hits the plane and where that makes up the Rect, then it checks if that hit is within the specified bounderies since a plane technically
//goes into infinity
impl Hittable for XyRect {
    fn hit(&self, r: &crate::ray::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        //At what t value does it hit the plane
        let t = (self.k - r.origin().z) / r.direction().z;
        if t > t_min && t < t_max {
            //What is the coordinates for the hit
            let x = r.origin().x + t*r.direction().x;
            let y = r.origin().y + t*r.direction().y;
            //check if within boundaries
            if(x > self.x0 && x < self.x1 && y>self.y0 && y < self.y1) {
                let outward_normal = Vec3::new(0.0, 0.0, 1.0);
                let mut rec = HitRecord::new(r.at(t), t);
                rec.set_face_normal(*r, outward_normal);
                rec.mat = self.mp;
                return Some(rec);
            }
        }
        return None;
    }
}