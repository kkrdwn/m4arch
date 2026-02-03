use std::fs;

use m4arch_core::error::{M4ArchError, Result};
use m4arch_core::keyboard::{
    brightness::KeyboardBrightness,
    rgb::{KeyboardRgb, Rgb},
};

const COLOR_PATH: &str = "/sys/class/leds/rgb:kbd_backlight/multi_intensity";
const BRIGHTNESS_PATH: &str = "/sys/class/leds/rgb:kbd_backlight/brightness";
const MAX_BRIGHTNESS_PATH: &str = "/sys/class/leds/rgb:kbd_backlight/max_brightness";

pub struct SysfsKeyboard;

impl KeyboardBrightness for SysfsKeyboard {
    fn get_brightness(&self) -> Result<u8> {
        let content = fs::read_to_string(BRIGHTNESS_PATH)?;
        content
            .trim()
            .parse()
            .map_err(|_| M4ArchError::InvalidValue)
    }

    fn get_max_brightness(&self) -> Result<u8> {
        let content = fs::read_to_string(MAX_BRIGHTNESS_PATH)?;
        content
            .trim()
            .parse()
            .map_err(|_| M4ArchError::InvalidValue)
    }

    fn set_brightness(&self, level: u8) -> Result<()> {
        fs::write(BRIGHTNESS_PATH, level.to_string())?;
        Ok(())
    }
}

impl KeyboardRgb for SysfsKeyboard {
    fn get_rgb(&self) -> Result<Rgb> {
        let content = fs::read_to_string(COLOR_PATH)?;
        let parts: Vec<&str> = content.trim().split_whitespace().collect();
        if parts.len() < 3 {
            return Err(M4ArchError::InvalidValue);
        }
        let r = parts[0].parse().map_err(|_| M4ArchError::InvalidValue)?;
        let g = parts[1].parse().map_err(|_| M4ArchError::InvalidValue)?;
        let b = parts[2].parse().map_err(|_| M4ArchError::InvalidValue)?;
        Ok(Rgb { r, g, b })
    }

    fn set_rgb(&self, rgb: Rgb) -> Result<()> {
        fs::write(COLOR_PATH, format!("{} {} {}", rgb.r, rgb.g, rgb.b))?;
        Ok(())
    }
}
