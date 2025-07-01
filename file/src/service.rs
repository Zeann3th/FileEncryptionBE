use crate::file::{self, Entity as FileEntity};
use crate::proto;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

#[derive(Debug, Default)]
pub struct FileService {
    db: DatabaseConnection,
}

impl FileService {
    pub fn new(db: DatabaseConnection) -> Self {
        FileService { db }
    }
}

#[allow(unused)]
#[tonic::async_trait]
impl proto::file_server::File for FileService {
    async fn get_all_files(
        &self,
        request: tonic::Request<proto::GetAllFilesRequest>,
    ) -> Result<tonic::Response<proto::GetAllFilesResponse>, tonic::Status> {
        let user_id = request.into_inner().user_id;

        let files = FileEntity::find()
            .filter(file::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .unwrap();

        let files = files
            .into_iter()
            .map(|file| proto::FileMetadata {
                id: file.id,
                name: file.name,
                size: file.size.parse().unwrap_or(0),
                encryption_method: file.encryption_method.to_string(),
                created_at: file.created_at.to_string(),
                updated_at: file.updated_at.to_string(),
            })
            .collect();

        Ok(tonic::Response::new(proto::GetAllFilesResponse { files }))
    }

    async fn get_file(
        &self,
        request: tonic::Request<proto::GetFileRequest>,
    ) -> Result<tonic::Response<proto::GetFileResponse>, tonic::Status> {
        let input = request.into_inner();

        let file = FileEntity::find()
            .filter(
                Condition::all()
                    .add(file::Column::Id.eq(input.file_id))
                    .add(file::Column::UserId.eq(input.user_id)),
            )
            .one(&self.db)
            .await;

        match file {
            Ok(Some(file)) => {
                let file = proto::FileMetadata {
                    id: file.id,
                    name: file.name,
                    size: file.size.parse().unwrap_or(0),
                    encryption_method: file.encryption_method.to_string(),
                    created_at: file.created_at.to_string(),
                    updated_at: file.updated_at.to_string(),
                };

                Ok(tonic::Response::new(proto::GetFileResponse {
                    file: Some(file),
                }))
            }

            Ok(None) => Err(tonic::Status::not_found("File not found")),

            Err(e) => {
                std::eprintln!("Error fetching file: {}", e);
                Err(tonic::Status::internal("Internal server error"))
            }
        }
    }

    async fn encrypt_file(
        &self,
        request: tonic::Request<proto::EncryptFileRequest>,
    ) -> Result<tonic::Response<proto::EncryptFileResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("NOT IMPLEMENTED"))
    }

    async fn download_encrypted_file(
        &self,
        request: tonic::Request<proto::GetFileRequest>,
    ) -> Result<tonic::Response<proto::DownloadEncryptedFileResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("NOT IMPLEMENTED"))
    }

    async fn decrypt_file(
        &self,
        request: tonic::Request<proto::DecryptFileRequest>,
    ) -> Result<tonic::Response<proto::DecryptFileResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("NOT IMPLEMENTED"))
    }

    async fn delete_file(
        &self,
        request: tonic::Request<proto::DeleteFileRequest>,
    ) -> Result<tonic::Response<proto::Empty>, tonic::Status> {
        let input = request.into_inner();

        let file = FileEntity::find()
            .filter(
                Condition::all()
                    .add(file::Column::Id.eq(input.file_id))
                    .add(file::Column::UserId.eq(input.user_id)),
            )
            .one(&self.db)
            .await;

        match file {
            Ok(Some(file)) => {
                let result = file.delete(&self.db).await;

                match result {
                    Ok(_) => Ok(tonic::Response::new(proto::Empty {})),
                    Err(e) => {
                        std::eprintln!("Error deleting file: {}", e);
                        Err(tonic::Status::internal("Internal server error"))
                    }
                }
            }

            Ok(None) => Err(tonic::Status::not_found("File not found")),

            Err(e) => {
                std::eprintln!("Error fetching file for deletion: {}", e);
                Err(tonic::Status::internal("Internal server error"))
            }
        }
    }
}
