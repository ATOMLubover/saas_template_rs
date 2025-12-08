use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetUserArgs {
    pub user_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GetUserReply {
    pub user_id: String,
    pub email: String,
    pub nickname: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginUserArgs {
    pub email: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginUserReply {
    pub user_id: String,
    pub token: String,
}
