use crate::enums::roles::Role;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub subject: i64,
    pub role: Role,
    pub exp: usize,
}

struct JwtConfig {
    secret_key: String,
    expiry: i64,
    expiry_for_30_days: i64,
    refresh_expiry: i64,
    refresh_expiry_for_30_days: i64
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct Token {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "expiresIn")]
    expires_in: i64,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "refreshTokenExpiresIn")]
    refresh_expires_in: i64,
}

fn get_jwt_config() -> JwtConfig {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiry = env::var("JWT_EXPIRY_IN_MINUTES")
        .expect("JWT_EXPIRY_IN_MINUTES must be set")
        .parse()
        .expect("JWT_EXPIRY_IN_MINUTES must be a valid integer");
    let expiry_for_30_days = env::var("JWT_EXPIRY_FOR_30_DAYS_IN_MINUTES")
        .expect("JWT_EXPIRY_FOR_30_DAYS_IN_MINUTES must be set")
        .parse()
        .expect("JWT_EXPIRY_FOR_30_DAYS_IN_MINUTES must be a valid integer");
    let refresh_expiry = expiry * 24;
    let refresh_expiry_for_30_days = expiry_for_30_days * 24;

    JwtConfig { secret_key, expiry, refresh_expiry, expiry_for_30_days, refresh_expiry_for_30_days }
}

pub fn create_jwt(subject: &i64, role: &Role, remember_me: bool) -> Token {
    let config = get_jwt_config();

    let access_expires_in = if !remember_me { 
        config.expiry
    } else {
        config.expiry_for_30_days
    };
    
    
    let access_expiration = Utc::now()
        .checked_add_signed(Duration::minutes(access_expires_in))
        .expect("Valid timestamp")
        .timestamp();

    let refresh_expires_in = if !remember_me { 
        config.refresh_expiry
    } else {
        config.refresh_expiry_for_30_days
    };
    
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
