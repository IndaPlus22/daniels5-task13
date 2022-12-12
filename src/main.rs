use std::alloc::System;
use std::fs::File;
use std::io::prelude::*;
mod vector;
use vector::*;
use vector::vector::Vec3;
fn main() -> std::io::Result<()> {
    let mut vec1 = Vec3::new(1.0,1.0,1.0);
    let vec2 = Vec3::new(2.0,3.0,1.0);
    //vec1+=vec2;
    println!("SUMMM: {}", Vec3::unit_vector(Vec3::cross(vec1, vec2)).length());


    let nx = 200;
    let ny = 100;
    let mut file = File::create("hello.ppm")?;
    write!(file, "P3\n{} {}\n", nx, ny)?;
    write!(file, "255\n")?;
    let mut j = nx;
    while j-1>= 0 {
        for i in 0..nx {
            let r = i as f64 /nx as f64;
            let g = j as f64 /ny as f64;
            let b = 0.2;
            let ir = (255.99*r) as i32;
            let ig = (255.99*g) as i32;
            let ib = (255.99*b) as i32;
            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
        j = j-1;
    }
    Ok(())


}
