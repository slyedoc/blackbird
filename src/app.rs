use crate::prelude::*;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Outlet, ParentRoute, ProtectedRoute, Route, Router, Routes},
    path,
};
//use leptos_use::{ColorMode, UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options};

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

    //provide_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>((color_mode, set_color_mode));

    // App setup based on https://tailwindcss.com/plus/ui-blocks/application-ui/application-shells/stacked
    view! {
      <Router>
        <AppNav logout />
        <div class="flex-grow py-10">
          <main class="mx-auto max-w-7xl px-4 pb-12 sm:px-6 lg:px-8">
            <Routes fallback=|| "Route not found.">
              <Route path=path!("/") view=Home />
              // <Route path=path!("games") view=Games/>
              <ParentRoute path=path!("/games") view=Games>
                <Route path=path!("unidir-events") view=UnidirEvents />
                <Route path=path!("sync-app") view=SyncApp />
                <Route path=path!(":id") view=GameProfile />
                <Route path=path!("") view=NoGame />
              </ParentRoute>
              <Route path=path!("todos") view=Todos />
              <Route path=path!("signup") view=move || view! { <SignupPage action=signup /> } />
              <Route path=path!("login") view=move || view! { <LoginPage action=login /> } />
              <ParentRoute path=path!("/users") view=Users>
                <Route path=path!(":id") view=UserProfile />
                <Route path=path!("") view=NoUser />
              </ParentRoute>
              <ProtectedRoute
                path=path!("/settings")
                condition=move || { user.get().map(|r| r.ok().flatten().is_some()) }
                redirect_path=|| "/"
                view=move || view! { <Settings logout /> }
              />
            </Routes>
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
        <h2>Users</h2>
        <p>add user list</p>

        <Outlet />
      </div>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    view! { <div>User Profile: <p>add user profile</p></div> }
}

#[component]
pub fn NoUser() -> impl IntoView {
    view! { <p>"Select a user."</p> }
}
