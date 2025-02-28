use {
    super::generic::{DataResponse, GenericResponse},
    axum::{http::StatusCode, response::IntoResponse, Json},
    sea_orm::DbErr,
    thiserror::Error,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // Database errors
    #[error("Database connection failed: {0}")]
    DatabaseConnectionFailed(#[source] DbErr),
    #[error("Database migration failed")]
    DatabaseMigrationFailed,
    #[error("Insert failed: {0}")]
    InsertFailed(#[source] DbErr),
    #[error("Query failed {0}")]
    QueryFailed(#[source] DbErr),
    #[error("Update failed: {0}")]
    UpdateFailed(#[source] DbErr),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Delete failed: {0}")]
    DeleteFailed(#[source] DbErr),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Login failed")]
    LoginFailed,

    // File errors
    #[error("Create file failed")]
    CreateFileFailed,
    #[error("File type invalid")]
    FileTypeInvalid,
    #[error("Field not found: {0}")]
    FieldNotFound(String),

    // Auth errors
    #[error("Please login first")]
    TokenNotFound,
    #[error("Hash password failed")]
    HashingFailed,
    #[error("Verify password failed")]
    VerifyPasswordFailed,
    #[error("Invalid credentials")]
    InvalidCredentials,

    // JWT errors
    #[error("JWT decode failed: {0}")]
    DecodeJwtFailed(#[source] jsonwebtoken::errors::Error),
    #[error("JWT encode failed: {0}")]
    EncodeJwtFailed(#[source] jsonwebtoken::errors::Error),

    // anyhow error
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    // Access denied
    #[error("Access denied")]
    AccessDenied,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Error::RecordNotFound => StatusCode::NOT_FOUND,
            Error::UserAlreadyExists => StatusCode::CONFLICT,
            Error::AccessDenied => StatusCode::FORBIDDEN,
            Error::InvalidCredentials => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = GenericResponse {
            status,
            result: DataResponse::<String> {
                msg: self.to_string(),
                data: None,
            },
        };

        (status, Json(body)).into_response()
    }
}
