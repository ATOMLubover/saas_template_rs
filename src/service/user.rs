use argon2::Argon2;
use argon2::password_hash::{
    Error as PasswordHashError, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    rand_core::OsRng,
};
use sqlx::{Executor, Postgres, query, query_as};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::jwt_codec::{JwtCodec, UserClaims};
use crate::repo::user::{BaseUser, UserSecrets};
use crate::result_trace::ResultTrace as _;
use crate::service::result::{accept, reject};
use crate::{
    model::user::{GetUserArgs, GetUserReply, LoginUserArgs, LoginUserReply},
    repo::Repo,
    service::result::{InterResult, ServiceResult},
};

fn generate_password_hash(password: &str) -> InterResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .trace_warn()?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, hash: &str) -> InterResult<bool> {
    let parsed_hash = PasswordHash::new(hash).trace_warn()?;

    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(err) => Err(err.into()),
    }
}

async fn select_user_secrets<'c, E>(exec: E, email: &str) -> InterResult<Option<UserSecrets>>
where
    E: 'c + Executor<'c, Database = Postgres>,
{
    let password_hash: Option<UserSecrets> = query_as(
        r#"
        SELECT f_user_id, f_password_hash
        FROM t_user
        WHERE f_email = $1
    "#,
    )
    .bind(email)
    .fetch_optional(exec)
    .await?;

    Ok(password_hash)
}

async fn register_new_user<'c, E>(
    exec: E,
    user_id: &str,
    email: &str,
    nickname: &str,
    password_hash: &str,
) -> InterResult<()>
where
    E: 'c + Executor<'c, Database = Postgres>,
{
    query(
        r#"
        INSERT INTO t_user (f_user_id, f_email, f_nickname, f_password_hash)
        VALUES ($1, $2, $3, $4)
    "#,
    )
    .bind(user_id)
    .bind(email)
    .bind(nickname)
    .bind(password_hash)
    .execute(exec)
    .await
    .trace_error()?;

    Ok(())
}

fn generate_token(user_id: &str, config: &AppConfig, jwt_codec: &JwtCodec) -> InterResult<String> {
    let claims = UserClaims::with_exp(user_id, config.jwt_expiration_seconds);

    let token = jwt_codec.encode(&claims).trace_error()?;

    Ok(token)
}

pub async fn login_user(
    args: LoginUserArgs,
    config: &AppConfig,
    jwt_codec: &JwtCodec,
    repo: &Repo,
) -> ServiceResult<LoginUserReply> {
    let mut trx = repo.pool().begin().await?;

    let secrets = match select_user_secrets(&mut *trx, &args.email).await? {
        None => {
            // If the user does not exist, register a new user.
            let user_id = Uuid::now_v7().to_string();

            let password_hash = generate_password_hash(&args.password)?;

            register_new_user(
                &mut *trx,
                &user_id,
                &args.email,
                &args.nickname,
                &password_hash,
            )
            .await?;

            // Generate a token for the new user.
            let token = generate_token(&user_id, config, jwt_codec)?;

            return Ok(accept()
                .with_code(204)
                .with_data(LoginUserReply { user_id, token }));
        }
        Some(p) => p,
    };

    // If the user exists, verify the password.

    if !(verify_password(&args.password, &secrets.f_password_hash)?) {
        // Unmatched password, return an error.
        return Ok(reject("Invalid password").with_code(400));
    }

    // Password matched, generate a token.
    let token = generate_token(&secrets.f_id, config, jwt_codec)?;

    Ok(accept().with_data(LoginUserReply {
        user_id: secrets.f_id,
        token,
    }))
}

pub async fn get_user_profile(args: GetUserArgs, repo: &Repo) -> ServiceResult<GetUserReply> {
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
        None => Ok(reject("User not found").with_code(404)),
        Some(u) => Ok(accept().with_data(GetUserReply {
            user_id: u.f_id,
            email: u.f_email,
            nickname: u.f_nickname,
            created_at: u.f_created_at,
        })),
    }
}
