mod pages;
mod style;

use iced::widget::{button, column, container, row, text, Rule};
use iced::{executor, Application, Command, Element, Font, Length, Settings, Theme};
use m4arch_service::{get_brightness, get_rgb_color, set_brightness, set_rgb_color};
use std::borrow::Cow;

// Definisi Font
const POPPINS: Font = Font {
    family: iced::font::Family::Name("Poppins"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

const ICON_FONT: Font = Font {
    family: iced::font::Family::Name("GeistMono Nerd Font Propo"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub fn main() -> iced::Result {
    M4ArchGui::run(Settings {
        default_font: POPPINS,
        fonts: vec![
            Cow::Borrowed(include_bytes!("../data/fonts/Poppins/Poppins-Regular.ttf")),
            Cow::Borrowed(include_bytes!(
                "../data/fonts/GeistMono/GeistMonoNerdFontPropo-Regular.otf"
            )),
        ],
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Keyboard,
    About,
}

#[derive(Debug, Clone, Copy)]
pub enum RgbComponent {
    Red,
    Green,
    Blue,
}

struct M4ArchGui {
    active_page: Page,
    error_message: Option<String>,
    brightness_text: String,
    brightness_value: u8,
    rgb_value: (u8, u8, u8),
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<(u8, (u8, u8, u8)), String>),
    PageSelected(Page),
    BrightnessChanged(u8),
    RgbChanged(RgbComponent, u8),
    PresetColorSelected(u8, u8, u8),
    OpenUrl(String),
}

impl Application for M4ArchGui {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let initial_state = Self {
            active_page: Page::Dashboard,
            error_message: None,
            brightness_text: "Loading...".to_string(),
            brightness_value: 0,
            rgb_value: (255, 255, 255), // Default White
        };

        (
            initial_state,
            Command::perform(load_initial_data(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("m4arch Settings")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Loaded(Ok((brightness, rgb))) => {
                self.brightness_value = brightness;
                self.brightness_text = format!("Current Brightness: {}", brightness);
                self.rgb_value = rgb;
                self.error_message = None;
            }
            Message::Loaded(Err(e)) => {
                self.error_message = Some(e);
                self.brightness_text = "Error loading data".to_string();
            }
            Message::PageSelected(page) => {
                self.active_page = page;
            }
            Message::BrightnessChanged(val) => {
                self.brightness_value = val;
                // Panggil service untuk mengubah hardware
                if let Err(e) = set_brightness(val) {
                    let err_msg = format!("Error: {}", e);
                    self.brightness_text = err_msg.clone();
                    self.error_message = Some(err_msg);
                } else {
                    self.brightness_text = format!("Current Brightness: {}", val);
                    self.error_message = None;
                }
            }
            Message::RgbChanged(component, val) => {
                let (mut r, mut g, mut b) = self.rgb_value;
                match component {
                    RgbComponent::Red => r = val,
                    RgbComponent::Green => g = val,
                    RgbComponent::Blue => b = val,
                }
                self.rgb_value = (r, g, b);
                if let Err(e) = set_rgb_color(r, g, b) {
                    self.error_message = Some(format!("Error setting color: {}", e));
                } else {
                    self.error_message = None;
                }
            }
            Message::PresetColorSelected(r, g, b) => {
                self.rgb_value = (r, g, b);
                if let Err(e) = set_rgb_color(r, g, b) {
                    self.error_message = Some(format!("Error setting color: {}", e));
                } else {
                    self.error_message = None;
                }
            }
            Message::OpenUrl(url) => {
                // Menggunakan xdg-open untuk membuka URL di browser default
                let _ = std::process::Command::new("xdg-open").arg(url).spawn();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        // --- Sidebar ---
        let sidebar = column![
            text("M4ARCH").size(24).font(ICON_FONT),
            Rule::horizontal(10),
            sidebar_button("\u{f0e4}", "Dashboard", Page::Dashboard, self.active_page),
            sidebar_button("\u{f11c}", "Keyboard", Page::Keyboard, self.active_page),
            sidebar_button("\u{f05a}", "About", Page::About, self.active_page),
        ]
        .spacing(10)
        .padding(10)
        .width(200)
        .height(Length::Fill);

        // --- Content Area ---
        let content = container(match self.active_page {
            Page::Dashboard => pages::dashboard::view(),
            Page::Keyboard => {
                pages::keyboard::view(self.brightness_value, &self.brightness_text, self.rgb_value)
            }
            Page::About => pages::about::view(),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20);

        // --- Layout Gabungan ---
        row![sidebar, Rule::vertical(1), content].into()
    }
}

async fn load_initial_data() -> Result<(u8, (u8, u8, u8)), String> {
    let brightness = get_brightness().map_err(|e| e.to_string())?;
    let rgb = get_rgb_color().map_err(|e| e.to_string())?;
    Ok((brightness, rgb))
}

fn sidebar_button<'a>(
    icon: &'a str,
    label: &'a str,
    page: Page,
    active_page: Page,
) -> Element<'a, Message> {
    let is_selected = page == active_page;

    button(
        row![text(icon).font(ICON_FONT).size(20), text(label)]
            .spacing(10)
            .align_items(iced::Alignment::Center)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(12)
    .on_press(Message::PageSelected(page))
    .style(iced::theme::Button::Custom(Box::new(
        style::SidebarButtonStyle {
            selected: is_selected,
        },
    )))
    .into()
}
