use crate::{Vec3, hittable::Hittable, Ray, HitRecord, material::Material};
pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere { center: center, radius: radius, mat: material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().squared_length();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.squared_length() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        let sqrtd = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if(root > t_min && t_max > root) {
            let mut rec = HitRecord::new(r.at(root), root);
            rec.mat = self.mat;
            let outward_normal = (rec.p - self.center) / self.radius;
            return Some(rec.set_face_normal(*r, outward_normal));
        }
        root = (-half_b + sqrtd) / a;
        if root > t_min && t_max > root {
            let mut rec = HitRecord::new(r.at(root), root);
            rec.mat = self.mat;
            let outward_normal = (rec.p - self.center) / self.radius;
            return Some(rec.set_face_normal(*r, outward_normal));
        }
        return None;

    }
}