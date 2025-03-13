use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] header: MaybeProp<String>,
    #[prop(into, optional)] footer: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
      <div class= move || format!("card divide-y {}", class.get().unwrap_or_default()) > 
        <Show when=move || header.get().is_some()>
          <div class="px-4 py-5 sm:px-6">
            {header.get().unwrap()}
          </div>
        </Show>        
        // <!-- We use less vertical padding on card headers on desktop than on body sections -->        
        <div class="px-4 py-5 sm:p-6">// <!-- Content goes here -->
         { children() }
        </div>
        <Show when=move || footer.get().is_some()>
            <div class="px-4 py-4 sm:px-6">
                {footer.get().unwrap()}            
            </div>
        </Show>
      </div>
    }
}
