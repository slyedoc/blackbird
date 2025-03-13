use crate::prelude::*;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};
use leptos_use::{
    use_color_mode_with_options, ColorMode, UseColorModeOptions, UseColorModeReturn,
};
use log::info;


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
      <!DOCTYPE html>
      <html lang="en" class="h-full bg-slate-200 dark:bg-slate-900 text-gray-900 dark:text-white">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <AutoReload options=options.clone() />
          <HydrationScripts options />
          // <link rel="stylesheet" href="/fonts/inter.css" />
          <link rel="stylesheet" id="leptos" href="/pkg/blackbird.css" />

          <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
          <MetaTags />
        </head>
        <body class="h-full flex flex-col">
          <App />
        </body>
      </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // user
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();
    let signup = ServerAction::<Signup>::new();
    let user = Resource::new(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(),
    );
    provide_context(user);

    // color mode
    let UseColorModeReturn {
        mode: color_mode,
        set_mode: set_color_mode,
        ..
    } = use_color_mode_with_options(UseColorModeOptions::default().custom_modes(vec![
        // custom colors in addition to light/dark
        "dim".to_string(),
        "cafe".to_string(),
    ]));
    provide_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>((color_mode, set_color_mode));

    info!("test: {}", "hello");
    // App setup based on https://tailwindcss.com/plus/ui-blocks/application-ui/application-shells/stacked
    view! {
      <Router>
        <AppNav logout />
        <div class="flex-grow py-10">
          // <header>
          // <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          // <h1 class="text-3xl font-bold tracking-tight text-white">Dashboard3</h1>
          // </div>
          // </header>
          <main class="mx-auto max-w-7xl px-4 pb-12 sm:px-6 lg:px-8">

            <FlatRoutes fallback=|| "Not found.">
              // Route
              <Route path=path!("") view=Home />
              <Route path=path!("games") view=Games />

              // Games
              // {
              // Game::iter()
              // .map(|game| {
              // view! {
              // <Route path=game.path()
              // view=move || {
              // view! {
              // {game}
              // // <BevyCanvas init=move || { breakout::init_bevy_app() } {..} class="bg-white dark:bg-black w-full" />
              // }
              // }
              // />
              // }
              // })
              // .
              // }
              <Route
                path=path!("breakout")
                view=move || {
                  view! { <GameCanvas game=Game::Breakout /> }
                }
              />
              <Route
                path=path!("tictactoe")
                view=move || {
                  view! { <GameCanvas game=Game::TicTacToe /> }
                }
              />
              <Route
                path=path!("unidir_events")
                view=move || {
                  #[cfg(feature = "unidir_events")]
                  view! { <UnidirEvents /> }
                  #[cfg(not(feature = "unidir_events"))]
                  view! { "Not included" }
                }
              />
              <Route
                path=path!("sync_app")
                view=move || {
                  #[cfg(feature = "unidir_events")]
                  view! { <SyncApp /> }
                  #[cfg(not(feature = "unidir_events"))]
                  view! { "Not included" }
                }
              />

              <Route path=path!("todos") view=Todos />
              <Route path=path!("signup") view=move || view! { <SignupPage action=signup /> } />
              <Route path=path!("login") view=move || view! { <LoginPage action=login /> } />
              <ParentRoute path=path!("/users") view=Users>
                <Route path=path!(":id") view=UserProfile />
              </ParentRoute>
              <ProtectedRoute
                path=path!("/settings")
                condition=move || { user.get().map(|r| r.ok().flatten().is_some()) }
                redirect_path=|| "/"
                view=move || {
                  view! { <Settings logout /> }
                }
              />
            </FlatRoutes>

          </main>

        </div>
        <AppFooter />
      </Router>
    }
}

#[component]
pub fn Users() -> impl IntoView {
    view! {
      <div>
        // the nested child, if any
        // don’t forget this!
        Users: <p>add user list</p> <Outlet />
      </div>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    view! { <div>User Profile: <p>add user profile</p></div> }
}

#[component]
pub fn NoUser() -> impl IntoView {
    view! { <div>No User</div> }
}
