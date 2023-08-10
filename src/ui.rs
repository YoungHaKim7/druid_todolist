use druid::{
    widget::{Button, Checkbox, Flex, Label, List, TextBox},
    Env, Widget, WidgetExt,
};

use crate::data::{TodoItem, TodoState};

pub fn ui_builder() -> impl Widget<TodoState> {
    let header = Flex::row()
        .with_flex_child(TextBox::new().lens(TodoState::new_text).expand_width(), 1.)
        .with_child(
            Button::new("->").on_click(|_ctx, data: &mut TodoState, _env| {
                if data.new_text.trim() != "" {
                    let text = data.new_text.clone();
                    data.new_text = "".to_string();
                    data.todos.push_front(TodoItem {
                        checked: false,
                        text,
                    })
                }
            }),
        );

    let todos = List::new(|| {
        Flex::row()
            .with_child(Checkbox::new("").lens(TodoItem::checked))
            .with_child(Label::new(|data: &TodoItem, _: &Env| data.text.clone()))
    })
    .lens(TodoState::todos)
    .scroll();

    Flex::column().with_child(header).with_flex_child(todos, 1.)
}
