use crate::jabcode;
use num_enum::IntoPrimitive;

#[derive(Default)]
pub struct WriteOptions {
    pub color_number: ColorNumber,
    pub master: Symbol,
    pub slaves: Vec<Symbol>,
    pub master_symbol_width: i32,
    pub master_symbol_height: i32,
    pub module_size: i32,
    pub color_space: ColorSpace,
}

impl WriteOptions {
    pub fn symbol_number(&self) -> usize {
        1 + self.slaves.len()
    }
}

#[derive(Default)]
pub struct Symbol {
    pub ecc_level: EccLevel,
    pub version: Version,
    pub position: i32,
}

#[derive(Default, Clone, Copy)]
pub struct Version {
    pub x: i32,
    pub y: i32,
}

impl Into<jabcode::jab_vector2d> for Version {
    fn into(self) -> jabcode::jab_vector2d {
        jabcode::jab_vector2d {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Copy, Clone, Default, IntoPrimitive)]
#[repr(u8)]
pub enum EccLevel {
    One = 1,
    Two = 2,
    #[default]
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
}

#[derive(Copy, Clone, Default, IntoPrimitive)]
#[repr(i32)]
pub enum ColorNumber {
    Four = 4,
    #[default]
    Eight = 8,
}

#[derive(Copy, Clone, Default, IntoPrimitive)]
#[repr(i32)]
pub enum ColorSpace {
    #[default]
    Rgb = 0,
    Cmyk = 1,
}
