use sqlx::query_as;

use crate::repo::user::BaseUser;
use crate::result_trace::ResultTrace as _;
use crate::service::result::{accept, reject};
use crate::{
    model::user::{GetUserArgs, GetUserReply},
    repo::Repo,
    service::result::ServiceResult,
};

pub async fn get_user(args: GetUserArgs, repo: &Repo) -> ServiceResult<GetUserReply> {
    // Query the database for the user by id
    let row: Option<BaseUser> = query_as(
        r#"
        SELECT f_id, f_email, f_nickname, f_created_at
        FROM t_user
        WHERE f_id = $1
    "#,
    )
    .bind(&args.user_id)
    .fetch_optional(repo.pool())
    .await
    .trace_error()?;

    match row {
        None => Err(reject(404, "User not found")),
        Some(u) => Ok(accept().with_data(GetUserReply {
            user_id: u.f_id,
            email: u.f_email,
            nickname: u.f_nickname,
            created_at: u.f_created_at,
        })),
    }
}
