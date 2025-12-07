use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    http::result::HttpResult,
    model::user::{GetUserArgs, GetUserReply, LoginUserArgs, LoginUserReply},
    service,
    state::AppState,
};

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserArgs>,
) -> HttpResult<LoginUserReply> {
    service::user::login_user(payload, state.config(), state.jwt_codec(), state.repo())
        .await
        .into()
}

pub async fn get_user_profile(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> HttpResult<GetUserReply> {
    service::user::get_user_profile(GetUserArgs { user_id }, state.repo())
        .await
        .into()
}
