use actix_service::forward_ready;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    body::EitherBody,
    Error,
    HttpMessage,
    error::ErrorUnauthorized,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, encode, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use std::env;

const IGNORE_ROUTES: [&str; 2] = ["/login", "/register"];

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    user_id: i32,
}

pub fn create_jwt(user_id: i32) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
        user_id,
    };
    let secret = env::var("BACKEND_SECRET_KEY").unwrap();
    let header = jsonwebtoken::Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("BACKEND_SECRET_KEY").unwrap();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ).map(|data| data.claims)
}


#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    fn init() {
        dotenv().ok();
        env::var("BACKEND_SECRET_KEY").expect("BACKEND_SECRET_KEY must be set");
    }

    #[test]
    fn test_jwt_roundtrip() {
        init();
        let token = create_jwt(123);
        let claims = validate_jwt(&token).expect("Token must be valid");
        assert_eq!(claims.user_id, 123);
    }

    #[test]
    fn test_invalid_jwt() {
        init();
        let bad = validate_jwt("totally.bad.token");
        assert!(bad.is_err());
    }
}