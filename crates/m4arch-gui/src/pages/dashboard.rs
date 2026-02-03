use crate::Message;
use iced::widget::{column, text};
use iced::Element;

pub fn view() -> Element<'static, Message> {
    column![
        text("Dashboard").size(30),
        text("Welcome to m4arch configuration center.")
    ]
    .spacing(20)
    .into()
}
