use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::storage::use_storage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BananaState {
    pub name: String,
    pub color: String,
    pub size: String,
    pub count: u32,
}

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let the_default = BananaState {
        name: "Banana".to_string(),
        color: "Yellow".to_string(),
        size: "Medium".to_string(),
        count: 0,
    };

    let (state, set_state, _) = use_storage(cx, "banana-state", the_default.clone());

    let (state2, ..) = use_storage(cx, "banana-state", the_default.clone());

    view! { cx,
        <input
            class="block"
            prop:value=move || state().name
            on:input=move |e| set_state.update(|s| s.name = event_target_value(&e))
            type="text"
        />
        <input
            class="block"
            prop:value=move || state().color
            on:input=move |e| set_state.update(|s| s.color = event_target_value(&e))
            type="text"
        />
        <input
            class="block"
            prop:value=move || state().size
            on:input=move |e| set_state.update(|s| s.size = event_target_value(&e))
            type="text"
        />
        <input
            class="block"
            prop:value=move || state().count
            value=move || state().count
            on:input=move |e| set_state.update(|s| s.count = event_target_value(&e).parse::<f64>().unwrap() as u32)
            type="number"
            min="0"
            step="1"
            max="1000"
        />

        <p>"Second "<b><code>"use_storage"</code></b>":"</p>

        <pre>
            { move || format!("{:#?}", state2()) }
        </pre>

        <Note>"The values are persistant. When you reload the page the values will be the same."</Note>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
