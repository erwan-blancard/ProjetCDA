use actix_service::forward_ready;
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, error::ErrorUnauthorized, http::Method, Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, encode, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};


const IGNORE_ROUTES: [&str; 6] = [
    "/login", "/register", "/account/reset-password", "/account/request-password-reset",
    "/swagger-ui", "/api-docs"
];


#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
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

    let secret = std::env::var("BACKEND_SECRET_KEY").unwrap();
    let header = jsonwebtoken::Header::new(Algorithm::HS256);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("BACKEND_SECRET_KEY").unwrap();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ).map(|data| data.claims)
}


// middleware to guard routes with JWT

// factory
pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService { service })
    }
}

// service
pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        // allow if method is OPTIONS
        if *req.method() == Method::OPTIONS {
            let fut = self.service.call(req);

            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        // if the path is in IGNORE_ROUTES then automatically passes
        for ignore_route in IGNORE_ROUTES.iter() {
            if req.path().starts_with(ignore_route) {
                let fut = self.service.call(req);

                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }

        let mut opt_token: Option<String> = None;

        // get JWT from either auth header or cookie

        if let Some(token_value) = req.headers().get("Authorization") {
            // extract JWT from Authorization header
            if let Ok(token_str) = token_value.to_str() {
                opt_token = Some(token_str.trim_start_matches("Bearer ").to_string());
            }
        
        } else if let Some(cookie) = req.cookie("token") {
            // extract JWT from cookies
            opt_token = Some(cookie.value().to_string());
        }

        if let Some(token) = opt_token {
            if let Ok(claims) = validate_jwt(&token) {
                // insert user_id in request extensions for later use in handlers
                req.extensions_mut().insert(claims.user_id);

                let fut = self.service.call(req);

                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }

        return Box::pin(async move { Err(ErrorUnauthorized("Invalid token")) });
    }
}
