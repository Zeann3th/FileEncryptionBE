use crate::proto;

#[derive(Debug, Default)]
pub struct FileService;

#[allow(unused)]
#[tonic::async_trait]
impl proto::file_server::File for FileService {
    async fn get_all_files(
        &self,
        request: tonic::Request<proto::GetAllFilesRequest>,
    ) -> Result<tonic::Response<proto::GetAllFilesResponse>, tonic::Status> {
        // DB fetch
        Ok(tonic::Response::new(proto::GetAllFilesResponse {
            files: vec![],
        }))
    }

    async fn get_file(
        &self,
        request: tonic::Request<proto::GetFileRequest>,
    ) -> Result<tonic::Response<proto::GetFileResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("NOT IMPLEMENTED"))
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
        Err(tonic::Status::unimplemented("NOT IMPLEMENTED"))
    }
}
