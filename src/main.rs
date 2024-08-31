mod ui;
mod state;

use druid::{AppLauncher, WindowDesc};

fn main() {
    let main_window = WindowDesc::new(ui::init())
        .title("Chat")
        .window_size((800.0, 800.0));

    let state = state::AppState {
        message: "".into()
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(state)
        .expect("Failed to launch application");
}
