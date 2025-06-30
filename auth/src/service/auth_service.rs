use tonic::{Request, Response, Status};

use crate::proto::auth_server::Auth;
use crate::proto::{SignInRequest, SignInResponse, SignUpRequest, SignUpResponse};

#[derive(Debug, Default)]
pub struct AuthService;

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_in(
        &self,
        request: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        let response = SignInResponse {
            access_token: format!("Welcome, {}!", request.into_inner().username),
            refresh_token: "dummy_refresh_token".to_string(),
        };
        Ok(Response::new(response))
    }

    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        let response = SignUpResponse {
            message: format!(
                "User {} created successfully!",
                request.into_inner().username
            ),
        };
        Ok(Response::new(response))
    }
}
