use std::{f64::consts::PI};

use crate::{vector::vector::Vec3, ray::ray::Ray, random::random_double};

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
    //The essential idee is that we are calculating a set of vectors that represent the plane that is the imaginary viewport of the camera
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3,vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = vfov*(PI/180.0);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = Vec3::unit_vector(lookfrom - lookat); //A vector pointing towards the lookat point from the lookfrom point
        let u = Vec3::unit_vector(Vec3::cross(vup, w)); //Get a orthogonal vector from the specified up direction vector and w
        let v = Vec3::cross(w, u); //Get the last orthogonal vector from w and u. u and v is the vectors making up the plane and w is orthogonal to the plane pointing towars the lookat
        let origin = lookfrom;
        let horizontal = viewport_width * u; //Scale the vectors according to the width and height
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
    //Using the vectors and a given s and t create a ray originating from the plane and in a direction using a linear combination of u and v and w
    pub fn get_ray(&self, s: f64, t: f64) -> Ray{
        //println!("{}", self.v.y);
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u*rd.x + self.v*rd.y;

        return Ray::new(self.origin +offset , self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset);
    }
}