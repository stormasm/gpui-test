use std::time::Duration;

use chrono::{DateTime, Local};
use gpui::*;

struct Wrapper {
    datum: View<Datum>,
}

impl Wrapper {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            datum: cx.new_view(|cx| {
                let app_state = cx.global::<AppState>();
                let global_store = app_state.global_store.clone();

                cx.observe(&global_store, |_, _, cx| {
                    cx.notify();
                })
                .detach();

                Datum::new()
            }),
        }
    }
}
impl Render for Wrapper {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .text_color(white())
            .flex()
            .flex_col()
            .gap_3()
            .justify_center()
            .items_center()
            .child(self.datum.clone())
    }
}
struct Datum {}
impl Datum {
    fn new() -> Self {
        Self {}
    }
}
impl Render for Datum {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let app_state = cx.global::<AppState>();
        let time = app_state.global_store.read(cx).time;

        div().flex().child(format!("time: {}", time))
    }
}

struct GlobalStore {
    time: DateTime<Local>,
}
impl GlobalStore {
    fn init(cx: &mut AppContext) -> Model<Self> {
        let global_store = cx.new_model(|_| GlobalStore::new());
        let a = global_store.clone();
        cx.spawn(|mut cx| async move {
            loop {
                let _ = a.update(&mut cx, |global, cx| {
                    global.time = Local::now();
                    cx.notify();
                });
                cx.background_executor().timer(Duration::from_secs(1)).await;
            }
        })
        .detach();

        global_store
    }
    fn new() -> Self {
        Self { time: Local::now() }
    }
}

struct AppState {
    global_store: Model<GlobalStore>,
}
impl Global for AppState {}
impl AppState {
    fn init(cx: &mut AppContext) {
        let global_store = GlobalStore::init(cx);
        cx.set_global(AppState::new(global_store));
    }
    fn new(global_store: Model<GlobalStore>) -> Self {
        Self { global_store }
    }
}

actions!(ford, [Quit]);

fn main() {
    App::new().run(|cx| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        AppState::init(cx);
        let _ = cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| Wrapper::new(cx))
        });
    })
}
