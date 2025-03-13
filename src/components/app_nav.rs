use leptos::prelude::*;
use leptos_use::ColorMode;



use crate::prelude::*;

const NAVIGATION: [(&str, &str); 3] = [
  ("Home", "/"),
  ("Games", "/games"),
  ("Todos", "/todos"),
];


#[component]
pub fn AppNav(logout: ServerAction<Logout>) -> impl IntoView {

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

    let (menu, set_menu) = signal(false);
    let menu_icon = Signal::derive(move || {
        if menu.get() {
            i::AiCloseOutlined
        } else {
            i::AiMenuOutlined
        }
    });
    let (settings, set_settings) = signal(false);

    view! {
      <nav class="section-primary">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div class="relative flex h-16 items-center justify-between">
            <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">

              // Mobile menu button
              {
                view! {
                  <button
                    type="button"
                    class="relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hi"
                    aria-controls="mobile-menu"
                    on:click=move |_| set_menu.update(|v| *v = !*v)
                    aria-expanded="false"
                  >
                    <span class="absolute -inset-0.5"></span>
                    <span class="sr-only">Open main menu</span>
                    <Icon icon=menu_icon />
                  </button>
                }
              }

            </div>
            <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
              <div class="flex shrink-0 items-center">
                <img
                  class="h-8 w-auto"
                  src="https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=600"
                  alt="Your Company"
                />
              </div>
              <div class="hidden sm:ml-6 sm:block">
                <div class="flex space-x-4">
                  {NAVIGATION
                    .iter()
                    .map(|(name, href)| {
                      view! {
                        <a
                          href=*href
                          class="rounded-md px-3 py-2 text-sm font-medium text-gray-500 dark:text-gray-300 hover:border-gray-300 hover:bg-gray-700 hover:text-gray-700 dark:hover:text-white"
                        >
                          {*name}
                        </a>
                      }
                    })
                    .collect_view()}
                </div>
              </div>
            </div>
            <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
              // dark mode
              <ThemeButton class="hidden md:inline-block" />

              // profile dropwdown
              <Transition>
                {move || {
                  match user.get() {
                    None => {
                      view! {
                        <a href="/signup" class="flex w-25 btn-primary mr-2">
                          "Signup"
                        </a>
                        <a href="/login" class="flex w-25 btn-primary">
                          "Login"
                        </a>
                      }
                        .into_any()
                    }
                    Some(user) => {
                      view! {
                        <div class="relative text-left hidden md:inline-block">
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
                              class=" absolute right-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-white ring-1 shadow-lg ring-black/5 focus:outline-hidden"
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

          </div>
        </div>

        // Mobile menu
        <Show when=move || { menu.get() }>
          <div class="sm:hidden">
            <div class="space-y-1 pt-2 pb-3">
              // Current: "border-indigo-500 bg-indigo-50 text-indigo-700", Default: "border-transparent text-gray-600 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-800"
              {NAVIGATION
                .iter()
                .map(|(name, href)| {
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
            <Transition>
              {move || {
                match user.get() {
                  None => {
                    view! {
                      <div class="border-t border-gray-700 pt-4 pb-3 flex flex-row justify-evenly">
                        <a href="/signup" class="flex w-25 btn-primary mr-2">
                          "Signup"
                        </a>
                        <a href="/login" class="flex w-25 btn-primary">
                          "Login"
                        </a>
                      </div>
                    }
                      .into_any()
                  }
                  Some(_user) => {
                    view! {
                      <div class="border-t border-gray-700 pt-4 pb-3">
                        <div class="flex items-center px-5 sm:px-6">
                          <div class="shrink-0">
                            <img
                              class="size-10 rounded-full"
                              src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
                              alt=""
                            />
                          </div>
                          <div class="ml-3">
                            <div class="text-base font-medium text-white">Tom Cook</div>
                            <div class="text-sm font-medium text-gray-400">tom@example.com</div>
                          </div>
                          <div class="relative ml-auto shrink-0">
                            <ThemeButton />
                            <button
                              type="button"
                              class=" rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hidden"
                            >
                              <span class="absolute -inset-1.5"></span>
                              <span class="sr-only">View notifications</span>
                              <Icon class="h-6 w-6" icon=i::AiBellOutlined />
                            </button>
                          </div>
                        </div>
                        <div class="mt-3 space-y-1 px-2 sm:px-3">
                          // Active: "bg-gray-100 outline-hidden", Not Active: ""
                          <a
                            href="/settings"
                            class="block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white"
                          >
                            "Settings"
                          </a>

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

          </div>
        </Show>

      </nav>
    }
}

#[component]
pub fn ThemeButton(
  #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
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
        class=format!("{} relative rounded-sm p-1 m-3 text-gray-800 dark:text-gray-300 hover:text-gray-300 dark:hover:text-white focus:ring-2 focus:outline-hidden", class.get().unwrap_or("".to_string()))
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
