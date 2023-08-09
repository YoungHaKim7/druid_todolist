use druid::{
    widget::{Button, Flex, Label, TextBox},
    Widget, WidgetExt,
};

use crate::data::TodoState;

pub fn ui_builder() -> impl Widget<TodoState> {
    let header = Flex::row()
        .with_flex_child(TextBox::new().lens(TodoState::new_text).expand_width(), 1.)
        .with_child(Button::new("->"));

    Flex::column().with_child(header)
}
