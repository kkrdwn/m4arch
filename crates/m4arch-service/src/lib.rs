use m4arch_core::error::{M4ArchError, Result};
use m4arch_core::keyboard::{BrightnessInfo, KeyboardBrightness, KeyboardRgb, Rgb};
use std::path::Path;

pub mod sysfs;
pub mod systemd;
pub mod tuxedo;
pub mod udev;

/// Trait gabungan untuk controller keyboard yang mendukung Brightness dan RGB.
pub trait KeyboardController: KeyboardBrightness + KeyboardRgb {}
impl<T: KeyboardBrightness + KeyboardRgb> KeyboardController for T {}

/// Mendeteksi dan mengembalikan driver keyboard yang sesuai.
pub fn get_keyboard() -> Option<Box<dyn KeyboardController>> {
    // 1. Cek driver spesifik Tuxedo terlebih dahulu (Prioritas Utama)
    if Path::new("/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/brightness").exists()
    {
        return Some(Box::new(tuxedo::TuxedoKeyboard));
    }

    // 2. Cek driver generik Sysfs / LED Class (Fallback)
    // Path ini biasanya symlink, jadi lebih aman dicek setelah driver spesifik
    if Path::new("/sys/class/leds/rgb:kbd_backlight/brightness").exists() {
        return Some(Box::new(sysfs::keyboard::SysfsKeyboard));
    }

    // 3. Tidak ditemukan driver yang cocok
    None
}

// === Facade Functions (API Publik untuk CLI/GUI) ===

fn get_controller() -> Result<Box<dyn KeyboardController>> {
    get_keyboard().ok_or_else(|| {
        M4ArchError::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No supported keyboard driver found",
        ))
    })
}

pub fn get_brightness() -> Result<u8> {
    get_controller()?.get_brightness()
}

pub fn get_max_brightness() -> Result<u8> {
    get_controller()?.get_max_brightness()
}

pub fn set_brightness(level: u8) -> Result<()> {
    get_controller()?.set_brightness(level)
}

pub fn increase_brightness(step: u8) -> Result<()> {
    get_controller()?.increase_brightness(step)
}

pub fn decrease_brightness(step: u8) -> Result<()> {
    get_controller()?.decrease_brightness(step)
}

pub fn brightness_info() -> Result<BrightnessInfo> {
    get_controller()?.brightness_info()
}

pub fn get_rgb_color() -> Result<(u8, u8, u8)> {
    let rgb = get_controller()?.get_rgb()?;
    Ok((rgb.r, rgb.g, rgb.b))
}

pub fn set_rgb_color(r: u8, g: u8, b: u8) -> Result<()> {
    get_controller()?.set_rgb(Rgb { r, g, b })
}
