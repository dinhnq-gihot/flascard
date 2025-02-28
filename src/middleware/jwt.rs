use {
    crate::{
        enums::error::{Error, Result},
        utils::jwt::decode_jwt,
    },
    axum::{
        extract::Request,
        http::{header::AUTHORIZATION, HeaderMap},
        middleware::Next,
        response::IntoResponse,
    },
};

#[axum::debug_middleware]
pub async fn check_jwt(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    let auth_header = headers.get(AUTHORIZATION).and_then(|h| h.to_str().ok());

    if let Some(auth) = auth_header {
        if auth.starts_with("Bearer ") {
            let token = &auth[7..]; // Remove "Bearer " prefix

            match decode_jwt(token.to_string()) {
                Ok(claims) => {
                    // Store claims in request extensions
                    request.extensions_mut().insert(claims);
                    return Ok(next.run(request).await);
                }
                Err(_) => return Err(Error::InvalidCredentials),
            }
        }
    }
    Err(Error::InvalidCredentials)
}
