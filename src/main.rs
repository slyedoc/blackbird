use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use blackbird::prelude::*;
        
        use axum::{
            body::Body as AxumBody,
            extract::{Path, State},
            http::Request,
            response::{IntoResponse, Response},
            routing::get,
            Router,
        };

        use axum_session::{SessionConfig, SessionLayer, SessionStore};
        use axum_session_auth::{AuthConfig, AuthSessionLayer};
        use axum_session_sqlx::SessionSqlitePool;

        use leptos::{config::get_configuration, logging::log};
        use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
        use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
        use tower_http::services::ServeDir;

        use axum::extract::FromRef;
        use leptos::prelude::LeptosOptions;
        use leptos_axum::AxumRouteListing;



        #[tokio::main]
        async fn main() {            

            simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

            let pool = SqlitePoolOptions::new()
                .connect("sqlite:db.db")
                .await
                .expect("Could not make pool.");

            // Auth section
            let session_config = SessionConfig::default().with_table_name("axum_sessions");
            let auth_config = AuthConfig::<i64>::default();
            let session_store = SessionStore::<SessionSqlitePool>::new(
                Some(SessionSqlitePool::from(pool.clone())),
                session_config,
            )
            .await
            .unwrap();

            if let Err(e) = sqlx::migrate!().run(&pool).await {
                eprintln!("{e:?}");
            }
            
            let conf = get_configuration(Some("Cargo.toml")).unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes = generate_route_list(blackbird::app::App);

            let app_state = AppState {
                leptos_options,
                pool: pool.clone(),
                routes: routes.clone(),
            };

            // build our application with a route
            let app = Router::new()
                .route(
                    "/api/*fn_name",
                    get(server_fn_handler).post(server_fn_handler),
                )
                .nest_service("/assets", ServeDir::new("assets"))
                .leptos_routes_with_handler(routes, get(leptos_routes_handler))
                .fallback(leptos_axum::file_and_error_handler::<AppState, _>(blackbird::app::shell))
                .layer(
                    AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone()))
                        .with_config(auth_config),
                )
                .layer(SessionLayer::new(session_store))
                .with_state(app_state);

            // run our app with hyper
            // `axum::Server` is a re-export of `hyper::Server`
            log!("listening on http://{}", &addr);
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }

                /// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
        /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
        #[derive(FromRef, Debug, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub pool: SqlitePool,
            pub routes: Vec<AxumRouteListing>,
        }

        #[axum::debug_handler]
        async fn server_fn_handler(
            State(app_state): State<AppState>,
            auth_session: AuthSession,
            path: Path<String>,
            request: Request<AxumBody>,
        ) -> impl IntoResponse {
            log!("{:?}", path);

            handle_server_fns_with_context(
                move || {
                    provide_context(auth_session.clone());
                    provide_context(app_state.pool.clone());
                },
                request,
            )
            .await
        }

        #[axum::debug_handler]
        async fn leptos_routes_handler(
            auth_session: AuthSession,
            state: State<AppState>,
            req: Request<AxumBody>,
        ) -> Response {
            let State(app_state) = state.clone();
            let handler = leptos_axum::render_route_with_context(
                app_state.routes.clone(),
                move || {
                    provide_context(auth_session.clone());
                    provide_context(app_state.pool.clone());
                },
                move || blackbird::app::shell(app_state.leptos_options.clone()),
            );
            handler(state, req).await.into_response()
        }
    }
    else {
        pub fn main() {}
    }
}
