use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::*, 
    *
};
use strum::IntoEnumIterator;
use crate::{auth::*, components::*, routes::*, error_template::ErrorTemplate};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="text-gray-900 antialiased leading-tight">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/blackbird.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body class="min-h-screen bg-gray-100">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
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

    provide_meta_context();

    // view! {
    //     <Router>
    //         <FlatRoutes fallback=|| "Page not found.">
    //             <Route path=StaticSegment("") view=Home/>
    //         </FlatRoutes>
    //     </Router>
    // }

    view! {
        <Router>        
            <Navbar />
            <main>
                <FlatRoutes fallback=|| "Not found.">
                    // Route
                    <Route path=path!("") view=move || view! { <Home /> }/>
                    <Route path=path!("games") view=move || view! { <Games /> }/>

                    // Games
                    <Route path=path!("breakout") view=move || view! { <PlayGame game=Game::Breakout /> } />                            
                    <Route path=path!("tictactoe") view=move || view! { <PlayGame game=Game::TicTacToe /> } />                            

                    <Route path=path!("todos") view=Todos/>   
                    <Route path=path!("signup") view=move || view! { <Signup action=signup/> }/>                    
                    <Route path=path!("login") view=move || view! { <Login action=login/> }/>
                    <ProtectedRoute
                        path=path!("settings")
                        condition=move || user.get().map(|r| r.ok().flatten().is_some())
                        redirect_path=|| "/"
                        view=move || { view! { <Settings action=logout/> } }
                    />

                </FlatRoutes>
            </main>
        </Router>
    }
}


