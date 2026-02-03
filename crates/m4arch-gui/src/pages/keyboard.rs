use crate::{style, Message, RgbComponent};
use iced::widget::{button, column, container, row, slider, text};
use iced::{Alignment, Color, Element};

pub fn view(brightness: u8, brightness_text: &str, rgb: (u8, u8, u8)) -> Element<'_, Message> {
    let (r, g, b) = rgb;

    column![
        text("Keyboard Settings").size(30),
        column![
            text("Brightness").size(20),
            text(brightness_text).size(14),
            slider(0..=255, brightness, Message::BrightnessChanged).style(
                iced::theme::Slider::Custom(Box::new(style::RgbSliderStyle {
                    color: Color::WHITE
                }))
            )
        ]
        .spacing(10),
        column![
            row![
                text("RGB Color").size(20),
                container(row![])
                    .width(30)
                    .height(30)
                    .style(iced::theme::Container::Custom(Box::new(
                        style::ColorPreviewStyle {
                            color: Color::from_rgb8(r, g, b)
                        }
                    )))
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            rgb_slider("Red", r, RgbComponent::Red),
            rgb_slider("Green", g, RgbComponent::Green),
            rgb_slider("Blue", b, RgbComponent::Blue),
            row![
                preset_button("Red", 255, 0, 0),
                preset_button("Green", 0, 255, 0),
                preset_button("Blue", 0, 0, 255),
                preset_button("White", 255, 255, 255),
            ]
            .spacing(10)
        ]
        .spacing(10)
    ]
    .spacing(30)
    .into()
}

fn rgb_slider(label: &str, value: u8, component: RgbComponent) -> Element<'_, Message> {
    let color = match component {
        RgbComponent::Red => Color::from_rgb8(255, 0, 0),
        RgbComponent::Green => Color::from_rgb8(0, 255, 0),
        RgbComponent::Blue => Color::from_rgb8(0, 0, 255),
    };

    row![
        text(label).width(60),
        slider(0..=255, value, move |v| Message::RgbChanged(component, v)).style(
            iced::theme::Slider::Custom(Box::new(style::RgbSliderStyle { color }))
        ),
        text(value.to_string()).width(40),
    ]
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}

fn preset_button(label: &str, r: u8, g: u8, b: u8) -> Element<'_, Message> {
    let color = Color::from_rgb8(r, g, b);

    button(text(label))
        .on_press(Message::PresetColorSelected(r, g, b))
        .padding(10)
        .style(iced::theme::Button::Custom(Box::new(
            style::PresetButtonStyle { background: color },
        )))
        .into()
}
