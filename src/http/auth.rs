use axum::{Json, extract::State};

use crate::{
    http::result::HttpResult,
    model::user::{LoginUserArgs, LoginUserReply},
    service,
    state::AppState,
};

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginUserArgs,
    responses(
        (status = 200, description = "User login successful", body = HttpResult<LoginUserReply>)
    ),
    tag = "Auth"
)]
pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserArgs>,
) -> HttpResult<LoginUserReply> {
    service::auth::login_user(payload, state.config(), state.jwt_codec(), state.repo())
        .await
        .into()
}
