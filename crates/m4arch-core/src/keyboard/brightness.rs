//! Modul untuk mengontrol kecerahan backlight keyboard.
//!
//! Modul ini menyediakan trait dan struktur data untuk kecerahan keyboard.
use crate::error::Result;
use serde::Serialize;

/// Trait untuk mengontrol kecerahan keyboard.
/// Dapat diimplementasikan oleh berbagai driver hardware (Tuxedo, Generic LED, dll).
pub trait KeyboardBrightness {
    fn get_brightness(&self) -> Result<u8>;
    fn get_max_brightness(&self) -> Result<u8>;
    fn set_brightness(&self, level: u8) -> Result<()>;

    fn increase_brightness(&self, step: u8) -> Result<()> {
        let current = self.get_brightness()?;
        let max = self.get_max_brightness()?;
        let new = current.saturating_add(step).min(max);
        self.set_brightness(new)
    }

    fn decrease_brightness(&self, step: u8) -> Result<()> {
        let current = self.get_brightness()?;
        let new = current.saturating_sub(step);
        self.set_brightness(new)
    }

    fn brightness_info(&self) -> Result<BrightnessInfo> {
        let current = self.get_brightness()?;
        let max = self.get_max_brightness()?;
        let percent = calculate_percent(current, max);
        let is_on = current > 0;
        Ok(BrightnessInfo {
            current,
            max,
            percent,
            is_on,
        })
    }
}

/// Struktur yang menyimpan informasi lengkap tentang status kecerahan.
#[derive(Debug, Clone, Serialize)]
pub struct BrightnessInfo {
    /// Brightness saat ini (raw value)
    pub current: u8,

    /// Brightness maksimum yang didukung hardware
    pub max: u8,

    /// Persentase brightness (0–100)
    pub percent: u8,

    /// Apakah backlight sedang aktif
    pub is_on: bool,
}

fn calculate_percent(current: u8, max: u8) -> u8 {
    if max == 0 {
        0
    } else {
        (current as u16 * 100 / max as u16) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_percent() {
        assert_eq!(calculate_percent(0, 255), 0);
        assert_eq!(calculate_percent(255, 255), 100);
        assert_eq!(calculate_percent(128, 255), 50);
        assert_eq!(calculate_percent(64, 255), 25);
    }

    #[test]
    fn test_calculate_percent_edge_cases() {
        assert_eq!(calculate_percent(0, 0), 0); // Max 0 should return 0%
        assert_eq!(calculate_percent(10, 0), 0); // Max 0 should return 0%
    }
}
