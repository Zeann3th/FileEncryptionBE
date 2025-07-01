use std::str::FromStr;

use crate::entity::file::{self, ActiveModel, EncryptionMethod, Entity as FileEntity};
use crate::proto;
use crate::service::cipher;
use base64::{Engine as _, engine::general_purpose};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

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
        let input = request.into_inner();

        let ec = cipher::get_cipher(&input.encryption_method);

        if let Some(file) = input.file {
            let (cipher_text, key, nonce) = ec.encrypt(&file.content);

            let size = file.content.len();

            let file_id = Uuid::new_v4().to_string();
            let timestamp = chrono::Utc::now().to_string();

            // Save cipher text in bucket

            FileEntity::insert(ActiveModel {
                id: Set(file_id.clone()),
                user_id: Set(input.user_id),
                name: Set(file.clone().name),
                size: Set(size.to_string()),
                encryption_method: Set(EncryptionMethod::from_str(&input.encryption_method)
                    .map_err(|_| tonic::Status::invalid_argument("Invalid encryption method"))?),
                created_at: Set(timestamp.clone()),
                updated_at: Set(timestamp.clone()),
            })
            .exec(&self.db)
            .await;

            Ok(tonic::Response::new(proto::EncryptFileResponse {
                file: Some(proto::FileMetadata {
                    id: file_id,
                    name: file.name,
                    size: size as u64,
                    encryption_method: input.encryption_method,
                    created_at: timestamp.clone(),
                    updated_at: timestamp,
                }),
                encryption_key: format!(
                    "{}::{}",
                    general_purpose::STANDARD.encode(nonce),
                    general_purpose::STANDARD.encode(key)
                ),
            }))
        } else {
            return Err(tonic::Status::invalid_argument("File is required"));
        }
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
                // Get file from bucket
                let data: Vec<u8> = vec![];

                let (nonce, key) = input.encryption_key.split_once("::").ok_or_else(|| {
                    tonic::Status::invalid_argument("Invalid encryption key format")
                })?;

                let nonce = match general_purpose::STANDARD.decode(nonce) {
                    Ok(n) => n,
                    Err(_) => return Err(tonic::Status::invalid_argument("Invalid base64 nonce")),
                };

                let key = match general_purpose::STANDARD.decode(key) {
                    Ok(k) => k,
                    Err(_) => return Err(tonic::Status::invalid_argument("Invalid base64 key")),
                };

                let ec = cipher::get_cipher(&file.encryption_method.to_string());

                let decrypted_data = ec.decrypt(&data, &key, &nonce);

                Ok(tonic::Response::new(proto::DecryptFileResponse {
                    file: Some(proto::FileBlob {
                        name: file.name,
                        content: decrypted_data,
                    }),
                }))
            }

            Ok(None) => Err(tonic::Status::not_found("File not found")),

            Err(e) => {
                std::eprintln!("Error fetching file for decryption: {}", e);
                Err(tonic::Status::internal("Internal server error"))
            }
        }
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
