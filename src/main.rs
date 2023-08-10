use data::TodoState;
use druid::{AppLauncher, WindowDesc};
use ui::ui_builder;

mod data;
mod ui;

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("My Todo App")
        .window_size((400., 400.))
        .resizable(true);
    AppLauncher::with_window(main_window)
        .launch(TodoState::default())
        .expect("failed app")
}
