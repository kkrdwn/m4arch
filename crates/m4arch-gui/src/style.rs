use iced::widget::{button, container, slider};
use iced::{Background, Border, Color, Theme};

pub struct SidebarButtonStyle {
    pub selected: bool,
}

impl button::StyleSheet for SidebarButtonStyle {
    type Style = Theme;

    fn active(&self, theme: &Self::Style) -> button::Appearance {
        let palette = theme.extended_palette();

        let (bg, text_color) = if self.selected {
            (palette.primary.strong.color, palette.primary.strong.text)
        } else {
            (Color::TRANSPARENT, palette.background.base.text)
        };

        button::Appearance {
            background: Some(Background::Color(bg)),
            text_color,
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    fn hovered(&self, theme: &Self::Style) -> button::Appearance {
        let palette = theme.extended_palette();

        let (bg, text_color) = if self.selected {
            (palette.primary.base.color, palette.primary.base.text)
        } else {
            (palette.background.weak.color, palette.background.base.text)
        };

        button::Appearance {
            background: Some(Background::Color(bg)),
            text_color,
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, theme: &Self::Style) -> button::Appearance {
        self.active(theme)
    }

    fn disabled(&self, _theme: &Self::Style) -> button::Appearance {
        button::Appearance::default()
    }
}

pub struct RgbSliderStyle {
    pub color: Color,
}

impl slider::StyleSheet for RgbSliderStyle {
    type Style = Theme;

    fn active(&self, _theme: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (self.color, Color::from_rgb(0.2, 0.2, 0.2)),
                width: 4.0,
                border_radius: 2.0.into(),
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 9.0 },
                color: self.color,
                border_width: 1.0,
                border_color: Color::WHITE,
            },
        }
    }

    fn hovered(&self, theme: &Self::Style) -> slider::Appearance {
        let active = self.active(theme);
        slider::Appearance {
            handle: slider::Handle {
                color: Color {
                    a: 0.8,
                    ..self.color
                },
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self, theme: &Self::Style) -> slider::Appearance {
        self.hovered(theme)
    }
}

pub struct ColorPreviewStyle {
    pub color: Color,
}

impl container::StyleSheet for ColorPreviewStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.color)),
            border: Border {
                radius: 8.0.into(), // Membuat sudut tumpul (rounded)
                width: 1.0,
                color: Color::WHITE,
            },
            ..Default::default()
        }
    }
}

pub struct PresetButtonStyle {
    pub background: Color,
}

/// Menentukan apakah sebuah warna tergolong "terang" atau "gelap".
fn is_light(color: Color) -> bool {
    // Formula untuk menghitung persepsi kecerahan (luminance)
    // Nilai r, g, b di sini adalah f32 antara 0.0 dan 1.0
    let luminance = (0.299 * color.r) + (0.587 * color.g) + (0.114 * color.b);

    // Jika luminance > 0.5, anggap warna terang
    luminance > 0.5
}

impl button::StyleSheet for PresetButtonStyle {
    type Style = Theme;

    fn active(&self, _theme: &Self::Style) -> button::Appearance {
        let text_color = if is_light(self.background) {
            // Gunakan teks hitam untuk background terang
            Color::BLACK
        } else {
            // Gunakan teks putih untuk background gelap
            Color::WHITE
        };

        button::Appearance {
            background: Some(Background::Color(self.background)),
            text_color,
            border: Border {
                radius: 6.0.into(),
                width: 1.0,
                color: Color {
                    a: 0.7,
                    ..Color::WHITE
                },
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _theme: &Self::Style) -> button::Appearance {
        let active = self.active(_theme);

        // Gelapkan sedikit warna background saat di-hover
        let hovered_background = Color {
            r: self.background.r * 0.9,
            g: self.background.g * 0.9,
            b: self.background.b * 0.9,
            a: self.background.a,
        };

        button::Appearance {
            background: Some(Background::Color(hovered_background)),
            ..active
        }
    }
}
