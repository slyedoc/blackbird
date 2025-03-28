use cfg_if::cfg_if;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub permissions: HashSet<String>,
}

// Explicitly is not Serialize/Deserialize!
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserPasshash(String);

impl Default for User {
    fn default() -> Self {
        let permissions = HashSet::new();

        Self {
            id: -1,
            username: "Guest".into(),
            permissions,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::prelude::*;

        pub use axum_session_auth::{Authentication, HasPermission};
        pub type AuthSession = axum_session_auth::AuthSession<User, i32, SessionDbPool, DbPool>;

        pub use async_trait::async_trait;
        pub use bcrypt::{hash, verify, DEFAULT_COST};

        impl User {
            pub async fn get_with_passhash(id: i32, pool: &DbPool) -> Option<(Self, UserPasshash)> {
                let sqluser = sqlx::query_as!( SqlUser, "SELECT * FROM users WHERE id = $1", id)
                    .fetch_one(pool)
                    .await
                    .ok()?;

                //lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
                let sql_user_perms = sqlx::query_as!(SqlPermissionTokens,
                    "SELECT token FROM user_permissions WHERE user_id = $1",
                    id
                )
                .fetch_all(pool)
                .await
                .ok()?;

                Some(sqluser.into_user(Some(sql_user_perms)))
            }

            pub async fn get(id: i32, pool: &DbPool) -> Option<Self> {
                User::get_with_passhash(id, pool)
                    .await
                    .map(|(user, _)| user)
            }

            pub async fn get_from_username_with_passhash(
                name: String,
                pool: &DbPool,
            ) -> Option<(Self, UserPasshash)> {
                let sqluser = sqlx::query_as!( SqlUser, "SELECT * FROM users WHERE username = $1", name)
                    .fetch_one(pool)
                    .await
                    .ok()?;

                //lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
                let sql_user_perms = sqlx::query_as!(SqlPermissionTokens,
                    "SELECT token FROM user_permissions WHERE user_id = $1",
                    sqluser.id
                )
                .fetch_all(pool)
                .await
                .ok()?;

                Some(sqluser.into_user(Some(sql_user_perms)))
            }

            pub async fn get_from_username(name: String, pool: &DbPool) -> Option<Self> {
                User::get_from_username_with_passhash(name, pool)
                    .await
                    .map(|(user, _)| user)
            }
        }

        #[derive(sqlx::FromRow, Clone)]
        pub struct SqlPermissionTokens {
            pub token: String,
        }

        #[async_trait]
        impl Authentication<User, i32, DbPool> for User {
            async fn load_user(userid: i32, pool: Option<&DbPool>) -> Result<User, anyhow::Error> {
                let pool = pool.unwrap();

                User::get(userid, pool)
                    .await
                    .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
            }

            fn is_authenticated(&self) -> bool {
                true
            }

            fn is_active(&self) -> bool {
                true
            }

            fn is_anonymous(&self) -> bool {
                false
            }
        }

        #[async_trait]
        impl HasPermission<DbPool> for User {
            async fn has(&self, perm: &str, _pool: &Option<&DbPool>) -> bool {
                self.permissions.contains(perm)
            }
        }

        #[derive(sqlx::FromRow, Clone)]
        pub struct SqlUser {
            pub id: i32,
            pub email: String,
            pub username: String,
            pub password: String,
            pub created_at: NaiveDateTime,
            pub last_login: NaiveDateTime,
        }

        impl SqlUser {
            pub fn into_user(
                self,
                sql_user_perms: Option<Vec<SqlPermissionTokens>>,
            ) -> (User, UserPasshash) {
                (
                    User {
                        id: self.id,
                        username: self.username,
                        permissions: if let Some(user_perms) = sql_user_perms {
                            user_perms
                                .into_iter()
                                .map(|x| x.token)
                                .collect::<HashSet<String>>()
                        } else {
                            HashSet::<String>::new()
                        },
                    },
                    UserPasshash(self.password),
                )
            }
        }
    }
}

#[server]
pub async fn foo() -> Result<String, ServerFnError> {
    Ok(String::from("Bar!"))
}

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use crate::todos::auth;

    let auth = auth()?;

    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    let (user, UserPasshash(expected_passhash)) =
        User::get_from_username_with_passhash(username, &pool)
            .await
            .ok_or_else(|| ServerFnError::new("User does not exist."))?;

    match verify(password, &expected_passhash)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    email: String,
    username: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    sqlx::query!(
        "INSERT INTO users (email, username, password) VALUES ($1,$2,$3)",
        email.clone(),
        username.clone(),
        password_hashed
    )
    .execute(&pool)
    .await?;

    let user = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("Signup failed: User does not exist."))?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect("/");

    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}
