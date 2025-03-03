use {
    crate::{
        debug,
        enums::{
            error::{Error, Result},
            generic::into_ok_response,
        },
        models::user::{
            DeleteRequest, LoginRequest, RegisterUserRequest, UpdateUserRequest, UserModel,
        },
        r#static::BLACKLIST_TOKEN_VEC,
        server::AppState,
        utils::jwt::{encode_jwt, Claims},
    },
    axum::{extract::State, response::IntoResponse, Extension, Json},
    bcrypt::{hash, verify, DEFAULT_COST},
    flashcard::only_role,
    std::sync::Arc,
};

pub struct UserHandler;

impl UserHandler {
    #[only_role("Staff")]
    pub async fn get_all_users(
        State(state): State<AppState>,
        Extension(claims): Extension<Claims>,
    ) -> Result<impl IntoResponse> {
        debug!("get_all_users: {claims:?}");

        let service = Arc::clone(&state.user_service);
        let users = service.get_all_users().await?;

        Ok(into_ok_response("success".into(), Some(users)))
    }

    pub async fn register_user(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUserRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("register_user: {payload:?}");
        let service = Arc::clone(&state.user_service);

        let RegisterUserRequest {
            email,
            password,
            name,
            role,
        } = payload;

        let hashed_password = hash(&password, DEFAULT_COST).map_err(|_| Error::HashingFailed)?;

        let user: UserModel = service
            .create_user(email, hashed_password, name, role)
            .await?
            .into();

        Ok(into_ok_response(
            "registered successfully".into(),
            Some(user),
        ))
    }

    pub async fn login(
        State(state): State<AppState>,
        Json(payload): Json<LoginRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("login request: {payload:?}");

        let service = Arc::clone(&state.user_service);
        let LoginRequest { email, password } = payload;

        if let Some(user) = service.get_by_email(email).await? {
            debug!("user: {user:?}");
            // Verify password
            if !verify(password, &user.password).map_err(|_| Error::VerifyPasswordFailed)? {
                return Err(Error::LoginFailed);
            }

            let jwt = encode_jwt(user.id, user.role.to_string())?;
            Ok(into_ok_response("Login successfully".into(), Some(jwt)))
        } else {
            Err(Error::LoginFailed)
        }
    }

    pub async fn update(
        State(state): State<AppState>,
        Extension(claims): Extension<Claims>,
        Json(payload): Json<UpdateUserRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("update request: {claims:?} {payload:?}");

        let service = Arc::clone(&state.user_service);
        let UpdateUserRequest {
            name,
            email,
            password,
            role,
        } = payload;

        let updated = service
            .update_user(claims.id, name, email, password, role)
            .await?;

        Ok(into_ok_response("Updated successfully".into(), updated))
    }

    pub async fn delete(
        State(state): State<AppState>,
        Json(payload): Json<DeleteRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("delete request: {payload:?}");

        let service = Arc::clone(&state.user_service);
        let DeleteRequest { id } = payload;
        service.delete_user(id).await?;

        Ok(into_ok_response(
            "Deleted successfully".into(),
            None::<String>,
        ))
    }

    pub async fn logout(Extension(token): Extension<String>) -> Result<impl IntoResponse> {
        debug!("logout: token: {token:?}");
        BLACKLIST_TOKEN_VEC.lock().push(token);

        Ok(into_ok_response(
            "Logout successfully".into(),
            None::<String>,
        ))
    }
}
