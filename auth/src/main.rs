use proto::auth_server::AuthServer;
use service::auth_service::AuthService;
use std::env;

mod service;

mod proto {
    tonic::include_proto!("auth");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let port = env::var("AUTH_PORT").unwrap_or_else(|_| "8001".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let auth_service = AuthService::default();

    println!("Auth service listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
