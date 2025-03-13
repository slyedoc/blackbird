use crate::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <Card>
        <div>
          <span class="inline-flex items-center justify-center rounded-md bg-indigo-500 p-2 shadow-lg">
            <Icon icon={i::AiCloseOutlined} {..} class="h-6 w-6 stroke-white" />
          </span>
        </div>
        <h3 class="mt-5  font-medium tracking-tight">Writes upside-down</h3>
        <p class="text-secondary mt-2 text-sm ">
          The Zero Gravity Pen can be used to write in any orientation, including upside-down. It even works in outer space.
        </p>
      </Card>

      <Card class="mt-4" header="Header" footer="Footer">
        "asdfasdf"
      </Card>

      <h1>Leptos-Use SSR Example</h1>
    }
}
