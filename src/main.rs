mod ui;
mod state;

use druid::{im::Vector, AppLauncher, WindowDesc};
use state::Message;

fn main() {
    let main_window = WindowDesc::new(ui::init())
        .title("Chat")
        .window_size((800.0, 800.0));

    let mut fake_history: Vector<Message> = Vector::new();
    // make fake records
    for i in 0..20 {
        fake_history.push_back(Message {
            text: format!("message {}", i),
            date: 0
        });
    }

    let state = state::AppState {
        message: "".into(),
        history: fake_history
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(state)
        .expect("Failed to launch application");
}
