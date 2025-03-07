use icondata as i; // list at https://carloskiki.github.io/icondata/
use leptos::{html::Html, prelude::*};
use leptos_icons::Icon;
use leptos_router::{components::*, *};
use leptos_use::{
    use_color_mode_with_options, use_cycle_list_with_options, use_preferred_dark, use_timestamp,
    ColorMode, UseColorModeOptions, UseColorModeReturn, UseCycleListOptions, UseCycleListReturn,
};

use crate::auth::*;

#[component]
pub fn AppNav(logout: ServerAction<Logout>) -> impl IntoView {
    const navigation: [(&str, &str, bool); 2] =
        [("Games", "/games", true), ("Todos", "/todos", false)];

    let user_resource = use_context::<Resource<Result<Option<User>, ServerFnError>>>()
        .expect("User context not found");
    // simplified version of the user resource
    let user = Signal::derive(move || {
        user_resource
            .get()
            .map(|user| match user {
                Err(e) => None,
                Ok(None) => None,
                Ok(Some(user)) => Some(user),
            })
            .flatten()
    });

    let (menu, set_menu) = signal(false);
    let (settings, set_settings) = signal(false);

    view! {
      // <Html {..} class=move || dark_mode.get().to_string() />
      <nav class="bg-white dark:bg-gray-800 ">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div class="flex h-16 justify-between">
            <div class="flex">
              <div class="flex shrink-0 items-center">
                <img
                  class="h-8 w-auto"
                  src="https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=600"
                  alt="Your Company"
                />
              </div>
              <div class="hidden sm:-my-px sm:ml-6 sm:flex sm:space-x-8">
                {navigation
                  .iter()
                  .map(|(name, href, current)| {
                    view! {
                      <a
                        href=*href
                        class="inline-flex items-center border-b-2 border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700"
                      >
                        {*name}
                      </a>
                    }
                  })
                  .collect_view()}
              </div>
            </div>
            <div class="hidden sm:ml-6 sm:flex sm:items-center">
              // dark mode
              <ThemeButton />

              // profile dropwdown
              <Transition>
                {move || {
                  let btn = "ml-6 inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600";
                  match user.get() {
                    None => {
                      view! {
                        <a class=btn href="/signup">
                          "Signup"
                        </a>
                        <a class=btn href="/login">
                          "Login"
                        </a>
                      }
                        .into_any()
                    }
                    Some(user) => {
                      view! {
                        <div class="relative inline-block text-left">
                          <div>
                            <button
                              type="button"
                              class="relative flex max-w-xs items-center rounded-full bg-gray-800 text-sm focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hidden"
                              id="menu-button"
                              aria-expanded="true"
                              aria-haspopup="true"
                              on:click=move |_| set_settings.update(|v| *v = !*v)
                            >
                              {user.username}
                              <Icon icon=i::AiDownOutlined />
                            </button>
                          </div>
                          <Show when=move || { settings() }>
                            <div
                              class="absolute right-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-white ring-1 shadow-lg ring-black/5 focus:outline-hidden"
                              role="menu"
                              aria-orientation="vertical"
                              aria-labelledby="menu-button"
                              tabindex="-1"
                            >
                              <div class="py-1" role="none">
                                // Active: "bg-gray-100 text-gray-900 outline-hidden", Not Active: "text-gray-700"
                                <a href="/settings" class="block px-4 py-2 text-sm text-gray-700">
                                  "Settings"
                                </a>

                                <ActionForm action=logout>
                                  <button
                                    type="submit"
                                    class="block w-full px-4 py-2 text-left text-sm text-gray-700"
                                    role="menuitem"
                                    tabindex="-1"
                                  >
                                    "Log Out"
                                  </button>
                                </ActionForm>
                              </div>
                            </div>
                          </Show>
                        </div>
                      }
                        .into_any()
                    }
                  }
                }}
              </Transition>
            </div>
            // Mobile menu button
            <div class="-mr-2 flex items-center sm:hidden">
              <button
                class="justify-center rounded-md bg-gray-800 p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hi"
                type="button"
                aria-controls="mobile-menu"
                on:click=move |_| set_menu.update(|v| *v = !*v)
                aria-expanded="false"
              >
                <span class="absolute -inset-0.5"></span>
                <span class="sr-only">Open main menu</span>
                {move || {
                  if menu.get() {
                    view! { <Icon icon=i::AiCloseOutlined /> }
                  } else {
                    view! { <Icon icon=i::AiMenuOutlined /> }
                  }
                }}
              </button>
            </div>
          </div>
        </div>

        // Mobile menu
        <Show when=move || { menu.get() }>
          <div class="sm:hidden">
            <div class="space-y-1 pt-2 pb-3">
              // Current: "border-indigo-500 bg-indigo-50 text-indigo-700", Default: "border-transparent text-gray-600 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-800"
              {navigation
                .iter()
                .map(|(name, href, current)| {
                  view! {
                    <a
                      href=*href
                      class="block border-l-4 border-transparent py-2 pr-4 pl-3 text-base font-medium text-gray-600 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-800"
                    >
                      {*name}
                    </a>
                  }
                })
                .collect_view()}
            </div>
            <ThemeButton />
          </div>
        </Show>

      </nav>
    }
}

#[component]
pub fn ThemeButton() -> impl IntoView {
    let (mode, set_mode) = use_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>()
        .expect("Color mode context not found");

    let icon = Signal::derive(move || {
        if mode.get() == ColorMode::Dark {
            i::BsSun
        } else {
            i::BsMoonStars
        }
    });

    view! {
      <button
        type="button"
        class="relative rounded-full p-1 m-3 text-gray-400 dark:bg-gray-900 hover:text-white focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hidden"
        on:click=move |_| {
          match mode.get() {
            ColorMode::Dark => set_mode.set(ColorMode::Light),
            ColorMode::Light => set_mode.set(ColorMode::Auto),
            ColorMode::Auto => set_mode.set(ColorMode::Dark),
            _ => set_mode.set(ColorMode::Dark),
          }
        }
      >
        <Icon height="24" width="24" icon=icon />
      </button>
    }
}
