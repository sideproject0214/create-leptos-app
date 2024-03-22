use http::status::StatusCode;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum fivemintwentiesfourError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl fivemintwentiesfourError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            fivemintwentiesfourError::NotFound => StatusCode::NOT_FOUND,
            fivemintwentiesfourError::InternalServerError => { StatusCode::INTERNAL_SERVER_ERROR }
        }
    }
}
