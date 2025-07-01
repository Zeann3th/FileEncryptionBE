mod file;
mod proto;
mod service;

use proto::file_server::FileServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("FILE_PORT").unwrap_or_else(|_| "8001".to_string());
    let addr = format!("0.0.0.0:{}", port).parse()?;

    let db_conn = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = sea_orm::Database::connect(db_conn).await.unwrap();

    let file_service = service::FileService::new(db.clone());

    println!("File service listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(FileServer::new(file_service))
        .serve(addr)
        .await?;

    Ok(())
}
