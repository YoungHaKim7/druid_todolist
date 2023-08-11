# druid_todolist

# Druid Font 기타 스타일 설정

https://docs.rs/druid/latest/druid/struct.Env.html

# Druid Tutorial: Making a Todo app in Rust GUI(이거 보고 만듬)

https://youtu.be/YEa2eq4HEU8

- Code 원본
  - https://github.com/Techno3d/TodoTutorialDruid.git

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

# Enter Pattern

```rust
struct Enter;

impl<W: Widget<TodoState>> Controller<TodoState, W> for Enter {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut TodoState,
        env: &druid::Env,
    ) {
    }
    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &TodoState,
        env: &druid::Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &TodoState,
        data: &TodoState,
        env: &druid::Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}

```
