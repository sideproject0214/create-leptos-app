use http::status::StatusCode;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum FivemintwentiesfourError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl FivemintwentiesfourError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            FivemintwentiesfourError::NotFound => StatusCode::NOT_FOUND,
            FivemintwentiesfourError::InternalServerError => { StatusCode::INTERNAL_SERVER_ERROR }
        }
    }
}
