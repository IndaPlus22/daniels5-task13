use crate::{vector::vector::Vec3, material::Material, hittable::{self, Hittable, HitRecord}, aabb::Aabb};

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat: Material,
}

impl MovingSphere {
    pub fn new(cen0: Vec3, cen1: Vec3, _time0: f64, _time1: f64, r: f64, m: Material) -> MovingSphere {
        MovingSphere { center0: cen0, center1: cen1, time0: _time0, time1: _time1, radius: r, mat: m }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0 + ((time - self.time0) / (self.time1 - self.time0))*(self.center1 - self.center0);
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &crate::ray::ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().squared_length();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.squared_length() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        let sqrtd = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if(root > t_min && t_max > root) {
            let mut rec = HitRecord::new(r.at(root), root);
            rec.mat = self.mat;
            let outward_normal = (rec.p - self.center(r.time())) / self.radius;
            return Some(rec.set_face_normal(*r, outward_normal));
        }
        root = (-half_b + sqrtd) / a;
        if root > t_min && t_max > root {
            let mut rec = HitRecord::new(r.at(root), root);
            rec.mat = self.mat;
            let outward_normal = (rec.p - self.center(r.time())) / self.radius;
            return Some(rec.set_face_normal(*r, outward_normal));
        }
        return None;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: Aabb) -> Option<Aabb> {
        let box0 = Aabb::new(self.center(time0) - Vec3::new(self.radius, self.radius, self.radius), self.center(time0) + Vec3::new(self.radius, self.radius, self.radius));
        let box1 = Aabb::new(self.center(time1) - Vec3::new(self.radius, self.radius, self.radius), self.center(time1) + Vec3::new(self.radius, self.radius, self.radius));
        return Some(Aabb::surrounding_box(box0, box1));
    }
}