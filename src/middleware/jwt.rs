use {
    crate::{
        enums::error::{Error, Result},
        r#static::BLACKLIST_TOKEN_VEC,
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
        if let Some(stripped) = auth.strip_prefix("Bearer ") {
            let token = stripped; // Remove "Bearer " prefix

            // If token in blacklist => error
            if BLACKLIST_TOKEN_VEC
                .lock()
                .binary_search(&token.to_string())
                .is_ok()
            {
                return Err(Error::InvalidCredentials);
            }

            match decode_jwt(token.to_string()) {
                Ok(claims) => {
                    // Store claims in request extensions
                    request.extensions_mut().insert(claims);
                    request.extensions_mut().insert(token.to_string());
                    return Ok(next.run(request).await);
                }
                Err(_) => return Err(Error::InvalidCredentials),
            }
        }
    }
    Err(Error::InvalidCredentials)
}
