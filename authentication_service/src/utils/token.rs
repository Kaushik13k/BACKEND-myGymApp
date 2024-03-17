use crate::models::tokens;
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken as jwt;
use jwt::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use std::env;

pub fn encode_token(email_id: String, username: String) -> String {
    dotenv().ok();
    let sercet_key = env::var("SECRET_KEY").expect("sercet_key must be set");
    let converted_sercet_key = sercet_key.as_ref();

    let expiration_hours = 120;
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::try_hours(expiration_hours).expect("valid hours"))
        .expect("valid timestamp");

    let my_claims = tokens::Token {
        email: email_id,
        username: username,
        exp: expiration_time.timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(converted_sercet_key),
    )
    .unwrap();

    token
}

pub fn decode_token(token: String) -> Result<tokens::Token, jwt::errors::Error> {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("secret_key must be set");
    let converted_secret_key = secret_key.as_ref();

    let token_data = decode::<tokens::Token>(
        &token,
        &DecodingKey::from_secret(converted_secret_key),
        &Validation::new(Algorithm::HS256),
    );
    token_data.map(|data| data.claims)
}

pub fn validate_token(token: String) -> std::option::Option<String> {
    info!("validating token...");
    match decode_token(token) {
        Ok(decoded_token) => {
            if (decoded_token.exp == 0)
                && (decoded_token.exp as i64) <= ((Utc::now().timestamp()) + 10 * 60)
            {
                info!("Token is expired. Refresh it.");
                None
            } else {
                info!("Token is validated.");
                Some(decoded_token.username)
            }
        }
        Err(_) => {
            info!("Failed to decode token.");
            None
        }
    }
}
