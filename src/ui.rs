use druid::widget::{Button, Container, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Scroll, TextBox};
use druid::{Color, Widget, WidgetExt};
use crate::state::{AppState, Message};

fn message_item() -> impl Widget<Message> {
    let mut date_sent = Label::dynamic(|data: &Message, _| format!("Date: {}", data.date.to_string().clone()));
    date_sent.set_text_color(Color::grey(0.5));

    let content = Flex::column()
        .with_child(Label::dynamic(|data: &Message, _| data.text.clone()))
        .with_spacer(5.0)
        .with_child(date_sent)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .padding(10.0)
        .background(Color::grey(0.2));
    
    content
}

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

fn message_container() -> impl Widget<AppState> {
    let message_list = List::new(|| {
        Container::new(message_item())
            .padding(5.0)
    })
    .expand_width()
    .lens(AppState::history);

    Scroll::new(message_list)
    .vertical() 
    .expand_width()
}

pub fn init() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_child(message_container(), 1.0)
        .with_spacer(20.0)
        .with_child(message_box())
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .padding(10.0)
        .expand_width()
}