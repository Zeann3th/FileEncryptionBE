mod service;
mod proto;

use proto::file_server::FileServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("FILE_PORT").unwrap_or_else(|_| "8001".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;

    let file_service = service::FileService::default();

    tonic::transport::Server::builder()
        .add_service(FileServer::new(file_service))
        .serve(addr)
        .await?;

    Ok(())
}
