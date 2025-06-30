use axum::{Router, routing::get};

pub fn auth_router() -> Router {
    Router::new()
        .route("/sign-up", get(index))
        .route("/sign-in", get(login))
}

async fn index() -> &'static str {
    "Auth index"
}

async fn login() -> &'static str {
    "Login endpoint"
}
