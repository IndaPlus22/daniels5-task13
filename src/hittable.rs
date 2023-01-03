use crate::Ray;
use crate::Vec3;
use crate::material::Material;
#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Material
}

impl HitRecord {
    pub fn new(p: Vec3, t: f64) -> HitRecord {
        HitRecord { p: p, normal: Vec3::new(0.0,0.0,0.0), t: t, front_face: false, mat: Material::default()}
    }
    pub fn set_face_normal(&mut self,r: Ray, outward_normal: Vec3) -> HitRecord{
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        if(self.front_face == true) {
            self.normal = outward_normal;
            return *self
        }else {
            self.normal = -1.0*outward_normal;
            return *self
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}