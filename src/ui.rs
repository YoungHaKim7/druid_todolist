use druid::{widget::Label, Widget};

use crate::data::TodoState;

pub fn ui_builder() -> impl Widget<TodoState> {
    Label::new("Hello world")
}
