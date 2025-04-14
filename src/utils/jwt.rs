use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use utoipa::ToSchema;
use crate::enums::roles::Role;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub subject: i64,
    pub role: Role,
    pub exp: usize,
}

struct JwtConfig {
    secret_key: String,
    expiry: i64,
    refresh_expiry: i64
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct Token {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
    refresh_expires_in: i64,
}

fn get_jwt_config() -> JwtConfig {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiry = env::var("JWT_EXPIRY_IN_MINUTES")
        .expect("JWT_EXPIRY_IN_MINUTES must be set")
        .parse()
        .expect("JWT_EXPIRY_IN_MINUTES must be a valid integer");
    let refresh_expiry = expiry * 24;

    JwtConfig { secret_key, expiry, refresh_expiry }
}

pub fn create_jwt(subject: &i64, role: &Role) -> Token {
    let config = get_jwt_config();

    let access_expires_in = config.expiry;
    let access_expiration = Utc::now()
        .checked_add_signed(Duration::minutes(access_expires_in))
        .expect("Valid timestamp")
        .timestamp();

    let refresh_expires_in = config.refresh_expiry;
    let refresh_expiration = Utc::now()
        .checked_add_signed(Duration::minutes(refresh_expires_in))
        .expect("Valid timestamp")
        .timestamp();

    let access_claims = Claims {
        subject: subject.to_owned(),
        role: role.to_owned(),
        exp: access_expiration as usize,
    };

    let refresh_claims = Claims {
        subject: subject.to_owned(),
        role: role.to_owned(),
        exp: refresh_expiration as usize,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(config.secret_key.as_bytes());

    let access_token = encode(&header, &access_claims, &encoding_key)
        .expect("Error creating access token");

    let refresh_token = encode(&header, &refresh_claims, &encoding_key)
        .expect("Error creating refresh token");

    Token {
        access_token,
        expires_in: access_expires_in,
        refresh_token,
        refresh_expires_in,
    }
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(get_jwt_config().secret_key.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}
