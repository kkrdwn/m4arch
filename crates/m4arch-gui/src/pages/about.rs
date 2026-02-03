use crate::Message;
use iced::widget::{button, column, text};
use iced::Element;

pub fn view() -> Element<'static, Message> {
    column![
        text("About M4ARCH").size(30),
        text(format!("Version: {}", env!("CARGO_PKG_VERSION"))).size(18),
        text("Management System for Arch Linux (Axioo)").size(16),
        text("Created by kkrdwn").size(14),
        button(text("View on GitHub"))
            .on_press(Message::OpenUrl(
                "https://github.com/kkrdwn/m4arch".to_string()
            ))
            .padding(10),
    ]
    .spacing(20)
    .into()
}
