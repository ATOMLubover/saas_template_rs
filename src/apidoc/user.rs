use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::user::get_user,
    ),
    tags(
        (name = "User", description = "User related endpoints")
    )
)]
pub struct UserApiDoc;
