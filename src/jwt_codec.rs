use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub type CodecResult<T> = Result<T, jsonwebtoken::errors::Error>;

/// JWT payload for user tokens.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    /// Subject (user id).
    pub sub: String,
    /// Issued-at (seconds since epoch).
    pub iat: i64,
    /// Expiration (seconds since epoch).
    pub exp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}

impl UserClaims {
    /// Create new claims for `sub` with a TTL in seconds from now.
    pub fn with_exp(sub: impl Into<String>, ttl_seconds: usize) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp();

        Self {
            sub: sub.into(),
            iat: now,
            exp: now + ttl_seconds as i64,
            extra: None,
        }
    }
}

/// JWT encoder/decoder using HMAC HS256.
#[derive(Debug)]
pub struct JwtCodec {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl JwtCodec {
    /// Create a new codec from a shared secret (HMAC HS256).
    pub fn new(secret: impl AsRef<[u8]>) -> Self {
        let secret = secret.as_ref();
        let encoding = EncodingKey::from_secret(secret);
        let decoding = DecodingKey::from_secret(secret);

        Self { encoding, decoding }
    }

    /// Encode claims into a compact JWT string. Uses HS256 by default.
    pub fn encode(&self, claims: &UserClaims) -> CodecResult<String> {
        let header = Header::new(Algorithm::HS256);

        encode(&header, claims, &self.encoding)
    }

    /// Decode and validate a JWT string, returning the contained `UserClaims`.
    /// Validation uses HS256 and checks expiration by default.
    pub fn decode(&self, token: &str) -> CodecResult<UserClaims> {
        let mut validation = Validation::new(Algorithm::HS256);

        validation.validate_exp = true;

        let token_data: jsonwebtoken::TokenData<UserClaims> =
            decode(token, &self.decoding, &validation)?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let secret = b"my-secret-which-should-be-long-and-random";
        let codec = JwtCodec::new(secret.as_ref());

        let claims = UserClaims::with_exp("user-123", 60);
        let token = codec.encode(&claims).expect("encode");
        let decoded = codec.decode(&token).expect("decode");

        assert_eq!(decoded.sub, claims.sub);
        assert!(decoded.exp >= claims.iat);
    }
}
