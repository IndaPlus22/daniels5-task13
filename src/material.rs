use crate::{hittable::HitRecord, ray::ray::Ray, vector::vector::Vec3, random::random_double};
#[derive(Clone, Copy)]
pub enum Material {
    Lambertian {albedo: Vec3},
    Metal {albedo: Vec3, fuzz: f64},
    Dielectric {ir: f64}
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian { albedo: Vec3::new(0.0, 0.0, 0.0) }
    }
}


pub fn scatter(material: &Material, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool{
    match material {
        &Material::Lambertian { albedo } => {
            let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
            if(scatter_direction.near_zero()) {
                scatter_direction = rec.normal;
            }
            *scattered = Ray::new(rec.p, scatter_direction);
            *attenuation = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz } => {
            let reflected = reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
            *scattered = Ray::new(rec.p, reflected + fuzz*Vec3::random_in_unit_sphere());
            *attenuation = albedo;
            return (Vec3::dot(scattered.direction(), rec.normal) > 0.0);
        }
        &Material::Dielectric { ir } => {
            *attenuation = Vec3::new(1.0, 1.0, 1.0);
            let refraction_ratio;
            if(rec.front_face == true) {
                refraction_ratio = 1.0/ir;
            } else {
                refraction_ratio = ir;
            }
            let unit_direction = Vec3::unit_vector(r_in.direction());
            let cos_theta = f64::min(Vec3::dot(-1.0*unit_direction, rec.normal), 1.0);
            let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

            let cannot_reftact = refraction_ratio*sin_theta > 1.0;
            let direction: Vec3;
            if(cannot_reftact || reflectance(cos_theta, refraction_ratio) > random_double(0.0, 1.0)) {
                direction = reflect(&unit_direction, &rec.normal);
            } else {
                direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
            }
            *scattered = Ray::new(rec.p, direction);
            return true;
        }
    }
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    return r0 + (1.0-r0)*f64::powi(1.0-cosine, 5);
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - 2.0*Vec3::dot(*v, *n)*(*n);
}


