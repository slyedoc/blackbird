use crate::prelude::*;
use cfg_if::cfg_if;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub user: Option<User>,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub completed: bool,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {

        pub fn pool() -> Result<DbPool, ServerFnError> {
            use_context::<DbPool>()
                .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
        }

        pub fn auth() -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>()
                .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
        }

        #[derive(sqlx::FromRow, Clone)]
        pub struct SqlTodo {
            id: i32,
            user_id: i32,
            title: String,
            created_at: NaiveDateTime,
            completed: bool,
        }

        impl SqlTodo {
            pub async fn into_todo(self, pool: &DbPool) -> Todo {
                Todo {
                    id: self.id,
                    user: User::get(self.user_id, pool).await,
                    title: self.title,
                    created_at: self.created_at,
                    completed: self.completed,
                }
            }
        }
    }
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use futures::future::join_all;

    let pool = pool()?;

    Ok(join_all(
        sqlx::query_as!( SqlTodo, "SELECT * FROM todos")
            .fetch_all(&pool)
            .await?
            .iter()
            .map(|todo: &SqlTodo| todo.clone().into_todo(&pool)),
    )
    .await)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    Ok(
        sqlx::query!("INSERT INTO todos (title, user_id, completed) VALUES ($1, $2, false)",
            title,
            id)            
            .execute(&pool)
            .await
            .map(|_| ())?,
    )
}

// The struct name and path prefix arguments are optional.
#[server]
pub async fn delete_todo(id: i32) -> Result<(), ServerFnError> {
    let pool = pool()?;

    Ok(sqlx::query!("DELETE FROM todos WHERE id = $1", id)        
        .execute(&pool)
        .await
        .map(|_| ())?)
}
