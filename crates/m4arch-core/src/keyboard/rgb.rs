use crate::error::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait KeyboardRgb {
    fn get_rgb(&self) -> Result<Rgb>;
    fn set_rgb(&self, rgb: Rgb) -> Result<()>;
}
