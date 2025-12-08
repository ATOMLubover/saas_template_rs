use axum::extract::{Path, State};

use crate::{
    http::result::HttpResult,
    model::user::{GetUserArgs, GetUserReply},
    service,
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/{user_id}",
    responses(
        (status = 200, description = "Get user profile successful", body = HttpResult<GetUserReply>)
    ),
    params(
        ("user_id" = String, Path, description = "The ID of the user to retrieve")
    ),
    tag = "User"
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> HttpResult<GetUserReply> {
    service::user::get_user(GetUserArgs { user_id }, state.repo())
        .await
        .into()
}
