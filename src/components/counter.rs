use leptos::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (num, set_num) = signal(0);

    let _ = Effect::watch(
        move || num.get(),
        move |num, prev_num, _| {
            leptos::logging::log!("Number: {}; Prev: {:?}", num, prev_num);
        },
        false,
    );

    set_num.set(1); // > "Number: 1; Prev: Some(0)"
    view! {
      <div>
        <button class="btn-primary" on:click=move |_| set_num.set(0)>
          "Clear"
        </button>
        <button class="btn-primary" on:click=move |_| *set_num.write() -= 1>
          "-1"
        </button>
        <span>"Value: " {num} "!"</span>
        <button class="btn-primary" on:click=move |_| set_num.update(|value| *value += 1)>
          "+1"
        </button>
      </div>
    }
}
