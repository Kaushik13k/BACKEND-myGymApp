use crate::models::tokens;
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken as jwt;
use jwt::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::env;

pub fn encode_token(email_id: String, username: String) -> String {
    dotenv().ok();
    let sercet_key = env::var("SECRET_KEY").expect("sercet_key must be set");
    let converted_sercet_key = sercet_key.as_ref();

    let expiration_hours = 360;
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

pub fn _decode_token(token: String) -> tokens::Token {
    dotenv().ok();
    let sercet_key = env::var("SECRET_KEY").expect("sercet_key must be set");
    let converted_sercet_key = sercet_key.as_ref();

    let token_data = decode::<tokens::Token>(
        &token,
        &DecodingKey::from_secret(converted_sercet_key),
        &Validation::new(Algorithm::HS256),
    )
    .unwrap();

    println!("{:?}", token_data.claims.email);
    token_data.claims
}
