use crate::auth::*;
use leptos::prelude::*;

#[component]
pub fn Settings(logout: ServerAction<Logout>) -> impl IntoView {
    let user_resource = use_context::<Resource<Result<Option<User>, ServerFnError>>>()
        .expect("User context not found");
    // simplified version of the user resource
    let user = Signal::derive(move || {
        user_resource
            .get()
            .map(|user| match user {
                Err(_) => None,
                Ok(None) => None,
                Ok(Some(user)) => Some(user),
            })
            .flatten()
    });

    view! {
      <h1>"Settings"</h1>

      <Transition>
        {move || {
          match user.get() {
            None => view! { "Not logged in" }.into_any(),
            Some(user) => {
              view! {
                <div class="flex">
                  <div class="flex-grow">
                    <h2>"Account"</h2>
                    <p>"You are logged in as: " {user.username}</p>
                  </div>
                  <div>
                    <ActionForm action=logout>
                      <button
                        type="submit"
                        class="block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white"
                        // class="block w-full px-4 py-2 text-left text-sm text-gray-700"
                        role="menuitem"
                        tabindex="-1"
                      >
                        "Log Out"
                      </button>
                    </ActionForm>
                  </div>
                </div>
              }
                .into_any()
            }
          }
        }}
      </Transition>
    }
}
