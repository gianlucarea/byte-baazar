use axum::{middleware, routing::{get, post}, Router};

use crate::{handlers::{login_user, register_user}, middleware::auth_middleware};


pub fn all_route(database_pool: sqlx::Pool<sqlx::Postgres> ) -> Router {
    Router::new()
    .route("/", get(|| async {"Hello Aldino!"}))
    .route("/register", post(register_user))
    .route("/login", post(login_user))
    .route("/protected", get(|| async {"Hello Logged User!"}).route_layer(middleware::from_fn(auth_middleware)))
    .with_state(database_pool)
}

