use m4arch_core::error::{M4ArchError, Result};
use m4arch_core::keyboard::{KeyboardBrightness, KeyboardRgb, Rgb};
use std::fs;

pub struct TuxedoKeyboard;

impl TuxedoKeyboard {
    // Path sysfs spesifik untuk driver Tuxedo
    const BRIGHTNESS_PATH: &'static str =
        "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/brightness";
    const MAX_BRIGHTNESS_PATH: &'static str =
        "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/max_brightness";
    const RGB_PATH: &'static str =
        "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/multi_intensity";
}

impl KeyboardBrightness for TuxedoKeyboard {
    fn get_brightness(&self) -> Result<u8> {
        let content = fs::read_to_string(Self::BRIGHTNESS_PATH)?;
        content
            .trim()
            .parse()
            .map_err(|_| M4ArchError::InvalidValue)
    }

    fn get_max_brightness(&self) -> Result<u8> {
        let content = fs::read_to_string(Self::MAX_BRIGHTNESS_PATH)?;
        content
            .trim()
            .parse()
            .map_err(|_| M4ArchError::InvalidValue)
    }

    fn set_brightness(&self, level: u8) -> Result<()> {
        fs::write(Self::BRIGHTNESS_PATH, level.to_string())?;
        Ok(())
    }
}

impl KeyboardRgb for TuxedoKeyboard {
    fn get_rgb(&self) -> Result<Rgb> {
        let content = fs::read_to_string(Self::RGB_PATH)?;
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
        let value = format!("{} {} {}", rgb.r, rgb.g, rgb.b);
        fs::write(Self::RGB_PATH, value)?;
        Ok(())
    }
}
