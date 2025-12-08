use argon2::{
    Argon2, PasswordHash, PasswordHasher as _, PasswordVerifier as _,
    password_hash::{Error as PasswordHashError, SaltString, rand_core::OsRng},
};
use sqlx::{Executor, Postgres, query, query_as};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    jwt_codec::{JwtCodec, UserClaims},
    model::user::{LoginUserArgs, LoginUserReply},
    repo::{Repo, user::UserSecrets},
    result_trace::ResultTrace as _,
    service::result::{InterResult, ServiceResult, accept, reject},
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
    let claims = UserClaims::with_exp(user_id, config.jwt_exp_seconds);

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
        return Err(reject(400, "Invalid password"));
    }

    // Password matched, generate a token.
    let token = generate_token(&secrets.f_id, config, jwt_codec)?;

    Ok(accept().with_data(LoginUserReply {
        user_id: secrets.f_id,
        token,
    }))
}
