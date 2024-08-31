use druid::widget::{Button, Flex, Label, MainAxisAlignment, TextBox};
use druid::{Widget, WidgetExt};
use crate::state::AppState;

fn message_box() -> impl Widget<AppState> {
    let textbox = TextBox::new()
        .with_placeholder("Enter message...")
        .lens(AppState::message)
        .expand_width();

    let send_button = Button::new("Send")
        .on_click(|_ctx, _data, _env| {
            println!("Send button clicked");
        });

    Flex::row()
        .with_flex_child(textbox, 1.0)
        .with_spacer(6.0)
        .with_child(send_button)
        .expand_width()
}


pub fn init() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Hello, World!"))
        .with_flex_child(message_box(), 1.0)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(10.0)
        .expand_width()
}