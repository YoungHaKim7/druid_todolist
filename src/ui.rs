use druid::{widget::Label, Widget};

pub fn ui_builder() -> impl Widget<u64> {
    Label::new("Hello world")
}
