use std::{env, error::Error};

use axum::Router;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().nest("/auth", routes::auth::auth_router());

    let port = env::var("GATEWAY_PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("Invalid port number");

    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
