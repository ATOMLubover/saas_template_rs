use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct BaseUser {
    pub f_id: String,
    pub f_nickname: String,
    pub f_email: String,
    pub f_created_at: OffsetDateTime,
}

pub struct NewUser {
    pub f_nickname: String,
    pub f_email: String,
    pub f_password_hash: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserSecrets {
    pub f_id: String,
    pub f_password_hash: String,
}
