use crate::vector::vector::Vec3;
#[derive(Clone, Copy)]



pub enum Texture {
    SolidColor {ColorValue: Vec3},
    CheckerTexture {_even: Vec3, _odd: Vec3}
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        match self {
            self::Texture::SolidColor {ColorValue} => {
                return *ColorValue;
            },
            self::Texture::CheckerTexture { _even, _odd } => {
                let sines = (10.0*p.x).sin() * (10.0*p.y).sin() * (10.0*p.z).sin();
                if(sines < 0.0) {
                    return *_odd;
                } else {
                    return *_even;
                }
            }
        }
    }
}

