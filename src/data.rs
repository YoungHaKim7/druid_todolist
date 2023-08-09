use druid::{Data, Lens};
use im::Vector;

#[derive(Clone, Data, Lens, Default)]
pub struct TodoState {
    pub todo: Vector<TodoItem>,
    pub new_text: String,
}

#[derive(Clone, Data, Lens)]
pub struct TodoItem {
    pub checked: bool,
    pub text: String,
}
