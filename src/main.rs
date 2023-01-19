
use std::fs::File;
use std::io::prelude::*;
mod vector;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod random;
mod material;
mod texture;
mod xy_rect;
mod xz_rect;
mod yz_rect;
use crate::yz_rect::*;
use crate::xz_rect::*;
use crate::xy_rect::*;
use crate::material::*;
use crate::random::*;
use crate::camera::*;
use ray::ray::Ray;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use texture::Texture;
use vector::vector::Vec3;
use crate::hittable::*;
use crate::sphere::*;
use crate::hittable_list::*;
use rand::Rng;
use indicatif::*;



fn ray_color(r: &Ray, world: &HittableList, depth: i32, background: Vec3) -> Vec3 {
    //Return background color if ray dosen't hit anything within the defined depth
    if depth <= 0 {

        return Vec3::new(0.0, 0.0, 0.0);
    }
    //If the ray hits any object that the closest one and calculate the desired pixel color
    if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
        let mut scattered: Ray = *r; //Initilizae varibles  
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0); //Initilizae varibles  
        let emitted = material::emitted(&rec.mat,  rec.p); //Get the emitted value from the object that the ray hit

        if(!material::scatter(&rec.mat, r, &rec, &mut attenuation, &mut scattered)) { // Calculate the scattered ray from hitting the object
            return emitted; //If the object dosen't scatter the ray then return the emittans
        }
        return emitted + attenuation * ray_color(&scattered, world, depth-1, background); //This bassicly does emittans + (the color value of the object)*(Ray_color of the scattered ray)
    }
    return background;
}

//Take the color values and write to a ppm file

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

fn main() -> std::io::Result<()> {
    //Set the necessary settings.
    let aspect_ratio = 1.0;
    let width = 200;
    let height = (width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;


    //Add the objects to the world list of hitables
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    let red = Material::Lambertian { albedo: Texture::SolidColor { ColorValue: Vec3::new(0.65, 0.05, 0.05) } };
    let white = Material::Lambertian { albedo: Texture::SolidColor { ColorValue: Vec3::new(0.73, 0.73, 0.73) } };
    let green = Material::Lambertian { albedo: Texture::SolidColor { ColorValue: Vec3::new(0.12, 0.45, 0.15) } };
    let light = Material::DiffuseLight { emit: Texture::SolidColor { ColorValue: Vec3::new(15.0, 15.0, 15.0) } };
    let sphere1 = Material::Lambertian { albedo: Texture::SolidColor { ColorValue: Vec3::new(1.0, 0.0, 0.0) } };
    let sphere2 = Material::Dielectric { ir: 1.5 };
    let sphere3 = Material::Metal { albedo: Vec3::new(0.1,0.2,0.3), fuzz: 0.2 };
    //Yanky
    list.push(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    list.push(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    list.push(Box::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    list.push(Box::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white)));
    list.push(Box::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));
    list.push(Box::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));
    list.push(Box::new(Sphere::new(Vec3::new(400.0, 100.0, 225.0), 100.0, sphere1)));
    list.push(Box::new(Sphere::new(Vec3::new(200.0, 100.0, 125.0), 100.0, sphere2)));
    list.push(Box::new(Sphere::new(Vec3::new(400.0, 100.0, 225.0), -99.0, sphere2)));
    list.push(Box::new(Sphere::new(Vec3::new(300.0, 150.0, 225.0+160.0), 150.0, sphere3)));



    








    let world = HittableList::new(list);

    //Camera
    let lookfrom = Vec3::new(278.0, 278.0, -800.0); //The look from position of the camera
    let lookat = Vec3::new(278.0, 278.0, 0.0); // THe look at position of the camera
    let vup = Vec3::new(0.0, 2.0, 0.0); // The defined up direction/Rotation of the camera
    let dist_to_focus = 10.0; //Focus distance
    let aperture = 0.1; //Aperture
    let camera = Camera::new(lookfrom, lookat, vup, 40.0, aspect_ratio);

    

  

    let mut file = File::create("hello.ppm")?;
    write!(file, "P3\n{} {}\n", width, height)?;
    write!(file, "255\n")?;
    let mut j = height - 1;
    
    let bar = ProgressBar::new((j*width) as u64);
    bar.set_style(ProgressStyle::default_bar().template("[{elapsed} elapsed] {wide_bar:.green/white} {percent}% [{eta} remaining] [rendering]").ok().unwrap());
    while j >= 0 {
        //println!("Scan lines left: {}",j);
        for i in 0..width {
            let pixel_color: Vec3 = (0..samples_per_pixel).into_par_iter().map(|_sample| { //Multithreads the pixel sampling.
                let mut rng = rand::thread_rng(); 
                let u = (i as f64 + rng.gen::<f64>()) / (width-1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (height-1) as f64;
                let r = camera.get_ray(u, v);
                ray_color(&r, &world, max_depth, Vec3::new(0.0, 0.0, 0.0))//Send out random rays close to the desired pixel to gather an avarage color value called antialiasing.
            }).sum();
            bar.inc(1);
            write_color(&file, pixel_color, samples_per_pixel)
        }
        j = j-1;

        
    }
    bar.finish();
    println!("DONE!");
    Ok(())


}
