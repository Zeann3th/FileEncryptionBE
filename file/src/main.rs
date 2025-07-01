mod entity;
mod proto;
mod service;

use proto::file_server::FileServer;
use service::file::FileService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("FILE_PORT").unwrap_or_else(|_| "8001".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;

    let db_conn = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = sea_orm::Database::connect(db_conn).await.unwrap();

    let file_service = FileService::new(db.clone());

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    println!("File service listening on {}", addr);

    Server::builder()
        .add_service(service)
        .add_service(FileServer::new(file_service))
        .serve(addr)
        .await?;

    Ok(())
}
