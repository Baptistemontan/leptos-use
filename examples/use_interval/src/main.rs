use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_interval, UseIntervalReturn};

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let UseIntervalReturn {
        counter,
        reset,
        is_active,
        pause,
        resume
    } = use_interval(cx, 200);

    let (counter_read, _) = create_signal(cx, move || counter());

    view! { cx,
        <div>
            <p>"Interval fired: " { counter_read() }</p>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
