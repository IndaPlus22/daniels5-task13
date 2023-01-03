use std::{f64::consts::PI};

use crate::{vector::vector::Vec3, ray::ray::Ray};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3,vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = vfov*(PI/180.0);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;
        Camera { origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            u: u,
            w: w,
            v: v,
            lens_radius: 0.0,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray{
        //println!("{}", self.v.y);
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u*rd.x + self.v*rd.y;

        return Ray::new(self.origin +offset , self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset);
    }
}