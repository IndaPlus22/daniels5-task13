use crate::{hittable::HitRecord, ray::ray::Ray, vector::vector::Vec3, random::random_double, texture::{self, Texture}};
#[derive(Clone, Copy)]

//The material for a object
pub enum Material {
    Lambertian {albedo: Texture},
    Metal {albedo: Vec3, fuzz: f64},
    Dielectric {ir: f64},
    DiffuseLight {emit: Texture}
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian { albedo: Texture::SolidColor { ColorValue: (Vec3::new(0.0, 0.0, 0.0)) }  }
    }
}

//The scatter function decides how the ray should scatter when hitting a material, the function mutates the scattared vector and returns true if the ray has changed direction.
//It also mutates the attenuation vector which provides the color information from the hitpoint
pub fn scatter(material: &Material, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool{
    match material {
        &Material::Lambertian { albedo } => {
            //For lamertian we bounce the ray away from the surface normal randomly.
            let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
            if(scatter_direction.near_zero()) {
                scatter_direction = rec.normal;
            }
            *scattered = Ray::new(rec.p, scatter_direction);
            *attenuation = albedo.value(rec.p);
            return true;
        }
        &Material::Metal { albedo, fuzz } => {
            //For Metal we only reflect the ray away from the surface given the normal and collect the color value of the object.
            //We can also simulate fuzziness whith fuzz varible that determines how much randomness we should add to the reflected ray
            let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
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
            //In real life glass materials both relfect and refract based on the direction of the ray.
            //Further more the refract algoritms sometimes dosen't have solutions based on the incoming angle and refraction indexes
            //Therefor when we can't refract we instead reflect and also reflect based on Shlick approximation.
            if(cannot_reftact || reflectance(cos_theta, refraction_ratio) > random_double(0.0, 1.0)) {
                direction = reflect(unit_direction, rec.normal);
            } else {
                direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
            }
            *scattered = Ray::new(rec.p, direction);
            return true;
        }
        //The DiffuseLight does not scatter the light
        &Material::DiffuseLight { emit } => {
            return false;
        }
    }
}

//Returns the emittance of a hit object with the color in a vector form.
//Note only returns a emmitance that is not black if it hits a Diffuselight material.
pub fn emitted(material: &Material, p: Vec3) -> Vec3{
    match material {
        &Material::DiffuseLight { emit } => {
            return emit.value(p)
        }
        _ => {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }
}

//Calculating reflectance based on Shlick approximation
pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    return r0 + (1.0-r0)*f64::powi(1.0-cosine, 5);
}
//Calculate the reflected ray based on the surface normal and a given ray that hits the surface.
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0*Vec3::dot(v, n)*(n);
}




