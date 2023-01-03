use std::alloc::System;
use std::f64::MAX;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;
use std::mem::Discriminant;
mod vector;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod random;
mod material;
use crate::material::*;
use crate::random::*;
use crate::camera::*;
use ray::ray::Ray;
use vector::vector::Vec3;
use crate::hittable::*;
use crate::sphere::*;
use crate::hittable_list::*;
use rand::Rng;



fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {

    if (depth <= 0) {

        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
        let mut scattered: Ray = *r;
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if (material::scatter(&rec.mat, r, &rec, &mut attenuation, &mut scattered)) {
            return attenuation * ray_color(&scattered, world, depth - 1)
        }
        return Vec3::new(0.0, 0.0, 0.0);
        
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5*(unit_direction.y + 1.0);
    return (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
}

fn write_color(mut fils: &File, pixel_color: Vec3, samples_per_pixel: i32){
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r*scale).sqrt();
    g = (g*scale).sqrt();
    b = (b*scale).sqrt();
    write!(fils, "{} {} {}\n", (256.0 * clamp(r, 0.0, 0.999)) as i32, (256.0*clamp(g, 0.0, 0.999)) as i32, (256.0*clamp(b, 0.0, 0.999)) as i32);
}


fn random_scene() -> HittableList {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    let ground_material = Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) };
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));
     for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Vec3::new(a as f64 + 0.9*random_double(0.0,1.0), 0.2, b as f64 + 0.9*random_double(0.0, 1.0));

            if(center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if(choose_mat < 0.8) {
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    let spehere_material = Material::Lambertian { albedo: albedo };
                    list.push(Box::new(Sphere::new(center, 0.2, spehere_material)));
                }
                else if(choose_mat < 0.95) {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let spehere_material = Material::Metal { albedo: albedo, fuzz: fuzz };
                    list.push(Box::new(Sphere::new(center, 0.2, spehere_material)));
                } else {
                    let sphere_material = Material::Dielectric { ir: 1.5 };
                    list.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
     }
     let material1 = Material::Dielectric { ir: 1.5 };
     list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));
     let material2 = Material::Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) };
     list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));
     let material3 = Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 };
     list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));
     return HittableList::new(list);
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 3.0 / 2.0;
    let width = 150;
    let height = (width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;


    let world = random_scene();

    //Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture,dist_to_focus);

    

  

    let mut file = File::create("hello.ppm")?;
    write!(file, "P3\n{} {}\n", width, height)?;
    write!(file, "255\n")?;
    let mut j = height - 1;
    let mut rng = rand::thread_rng();
    while j >= 0 {
        println!("Scan lines left: {}",j);
        for i in 0..width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                
                let u = (i as f64 + rng.gen::<f64>()) / (width-1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (height-1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(&file, pixel_color, samples_per_pixel)
        }
        j = j-1;
    }
    println!("DONE!");
    Ok(())


}
