use crate::Ray;
use crate::Vec3;
use crate::material::Material;
#[derive(Clone, Copy)]

//This stores necessary information that is collected when a ray hits an object, such as the normal, material, hitpoiny
pub struct HitRecord {
    pub p: Vec3, //Hitpoint of the object
    pub normal: Vec3, //The surfance normal of the object
    pub t: f64, //The t value of the ray
    pub front_face: bool,
    pub mat: Material // Material of the object
}

impl HitRecord {
    pub fn new(p: Vec3, t: f64) -> HitRecord {
        HitRecord { p: p, normal: Vec3::new(0.0,0.0,0.0), t: t, front_face: false, mat: Material::default()}
    }
    pub fn set_face_normal(&mut self,r: Ray, outward_normal: Vec3) -> HitRecord{ //Decide weather the normal is facing away from the object or inwards, if faving inwards change to the negative direction.
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
//Impl the trait functions
pub trait Hittable : Sync{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}