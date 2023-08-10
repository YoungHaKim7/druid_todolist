# druid_todolist


# Druid Tutorial: Making a Todo app in Rust GUI(이거 보고 만듬)

https://youtu.be/YEa2eq4HEU8

# Druid Rutine

```rust
use druid::Widget;

use crate::data::TodoState;

pub struct Saver;

impl Widget<TodoState> for Saver {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut TodoState,
        env: &druid::Env,
    ) {
        todo!()
    }
    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &TodoState,
        env: &druid::Env,
    ) {
        todo!()
    }
    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &TodoState,
        data: &TodoState,
        env: &druid::Env,
    ) {
        todo!()
    }
    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &TodoState,
        env: &druid::Env,
    ) -> druid::Size {
        todo!()
    }
    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &TodoState, env: &druid::Env) {
        todo!()
    }
}

```
