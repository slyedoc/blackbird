use leptos::prelude::*;
use leptos_use::{UseToggleReturn, use_toggle};

#[derive(Clone)]
pub struct DisclosureContext<F, S>
where
    F: Fn() + Clone + Send + Sync + 'static,
    S: Fn() + Clone + Send + Sync + 'static,
{
    pub value: Signal<bool>,
    pub set_value: WriteSignal<bool>,
    pub toggle: F,
    pub close: S,
}

//   <Disclosure as="nav" class="bg-gray-800" v-slot="{ open }">
#[component]
pub fn Disclosure(
    #[prop(default = false)] default_open: bool,

    children: ChildrenFn,
) -> impl IntoView {
    let UseToggleReturn {
        value,
        set_value,
        toggle,
        ..
    } = use_toggle(default_open);

    let close = move || {
        if value.get() {
            return;
        }
        set_value.set(false);
    };

    let api = DisclosureContext {
        value: value.clone(),
        set_value: set_value.clone(),
        toggle: toggle.clone(),
        close: close,
    };

    provide_context(api);

    let children = StoredValue::new(children);

    view! {
       <div>
           <button on:click={move |_| (toggle)()}>
               Toggle
           </button>

           <Show when=move || value.get()>
               {children.read_value()()}
           </Show>
       </div>
    }
}
