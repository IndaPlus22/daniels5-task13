use crate::vector::vector::Vec3;
#[derive(Clone, Copy)]


//Simple texture enum, either a checkerd texture or a solid color
pub enum Texture {
    SolidColor {ColorValue: Vec3},
    CheckerTexture {_even: Vec3, _odd: Vec3}
}

impl Texture {
    //This returns the color value at a given point on the texture
    pub fn value(&self, p: Vec3) -> Vec3 {
        match self {
            //Solid colors always give the samel color value
            self::Texture::SolidColor {ColorValue} => {
                return *ColorValue;
            },
            //The checkered value is decided depending on the hitpoint.
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

