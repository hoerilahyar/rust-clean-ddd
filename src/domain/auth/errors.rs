use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid credential")]
    InvalidCredential,

    #[error("user not found")]
    UserNotFound,

    #[error("user inactive")]
    UserInactive,

    #[error("role not found")]
    RoleNotFound,

    #[error("invalid refresh token")]
    InvalidRefreshToken,

    #[error("refresh token expired")]
    RefreshTokenExpired,
}
