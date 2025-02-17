# druid_todolist

# Druid Font 기타 스타일 설정

https://docs.rs/druid/latest/druid/struct.Env.html

- .configure_env에서 만져주면 된다.

```rust
fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("My Todo App")
        .window_size((800., 800.))
        .resizable(true);

    let stored = read_stored();
    let default_state = TodoState {
        todos: Vector::from(stored.tasks),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, _state| {
            env.set(theme::UI_FONT, FontDescriptor::default().with_size(30.0));
            env.set(BUTTON_DARK, Color::rgba8(100, 100, 120, 0));
            env.set(BUTTON_LIGHT, Color::rgba8(100, 100, 120, 100));
            env.set(WINDOW_BACKGROUND_COLOR, Color::rgba8(0, 0, 0, 100));
        })
        .launch(default_state)
        .expect("failed app")
}
```

- 다른 설정들
  - https://docs.rs/druid/latest/src/text/text.rs.html#95
  - https://docs.rs/druid/latest/druid/text/enum.Attribute.html

# Druid Tutorial: Making a Todo app in Rust GUI(이거 보고 만듬)

https://youtu.be/YEa2eq4HEU8

- Code 원본
  - https://github.com/Techno3d/TodoTutorialDruid.git

# Druid Routine

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

# json 저장위치

```rs
    /// Returns the path to the user's cache directory.
    ///
    /// |Platform | Value                               | Example                      |
    /// | ------- | ----------------------------------- | ---------------------------- |
    /// | Linux   | `$XDG_CACHE_HOME` or `$HOME`/.cache | /home/alice/.cache           |
    /// | macOS   | `$HOME`/Library/Caches              | /Users/Alice/Library/Caches  |
    /// | Windows | `{FOLDERID_LocalAppData}`           | C:\Users\Alice\AppData\Local |
    pub fn cache_dir(&self) -> &Path {
        self.cache_dir.as_path()
    }
    /// Returns the path to the user's config directory.
    ///
    /// |Platform | Value                                 | Example                                  |
    /// | ------- | ------------------------------------- | ---------------------------------------- |
    /// | Linux   | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/alice/.config                      |
    /// | macOS   | `$HOME`/Library/Application Support   | /Users/Alice/Library/Application Support |
    /// | Windows | `{FOLDERID_RoamingAppData}`           | C:\Users\Alice\AppData\Roaming           |
    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }
    /// Returns the path to the user's local config directory.
    ///
    /// |Platform | Value                                 | Example                                  |
    /// | ------- | ------------------------------------- | ---------------------------------------- |
    /// | Linux   | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/alice/.config                      |
    /// | macOS   | `$HOME`/Library/Application Support   | /Users/Alice/Library/Application Support |
    /// | Windows | `{FOLDERID_LocalAppData}`           | C:\Users\Alice\AppData\Local               |
    pub fn config_local_dir(&self) -> &Path {
        self.config_local_dir.as_path()
    }

```
