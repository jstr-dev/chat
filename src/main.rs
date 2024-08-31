use druid::widget::{Container, Flex, Label, MainAxisAlignment};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};

fn init_ui() -> impl Widget<()> {
    let flex_box = Flex::column()
        .with_child(Label::new("Hello, World!"))
        .with_child(Label::new("Bottom"))
        .main_axis_alignment(MainAxisAlignment::SpaceBetween);

    Container::new(flex_box)
        .padding(10.0)
}

fn main() {
    let main_window = WindowDesc::new(init_ui())
        .title("Chat")
        .window_size((400.0, 300.0));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
        .expect("Failed to launch application");
}
