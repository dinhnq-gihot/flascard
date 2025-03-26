use {
    crate::enums::error::{Error, Result},
    chrono::{Duration, Utc},
    jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation},
    serde::{Deserialize, Serialize},
    std::env,
    uuid::Uuid,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub id: Uuid,
    pub role: String,
    pub exp: usize,
}

pub fn encode_jwt(user_id: Uuid, user_role: String) -> Result<String> {
    let claims = Claims {
        id: user_id,
        role: user_role,
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
    };

    let secret = env::var("JWT_SECRET").map_err(|_| Error::EnvVarNotFound("JWT_SECRET".into()))?;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(Error::EncodeJwtFailed)
}

pub fn decode_jwt(token: String) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").map_err(|_| Error::EnvVarNotFound("JWT_SECRET".into()))?;
    let key = DecodingKey::from_secret(secret.as_bytes());

    let claims = decode(&token, &key, &Validation::default())
        .map_err(Error::DecodeJwtFailed)?
        .claims;

    Ok(claims)
}
