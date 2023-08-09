use druid::{AppLauncher, WindowDesc};
use ui::ui_builder;

mod data;
mod ui;

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("My Todo App")
        .window_size((400., 400.));
    AppLauncher::with_window(main_window)
        .launch(0)
        .expect("failed app")
}
