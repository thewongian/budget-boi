use super::error::*;
use super::mongo::Db;
use crate::server::mongo::User;
use chrono::Utc;
use jsonwebtoken::{
    decode, encode,
    Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use mongodb::bson::{self, doc, Document, oid::ObjectId};
use serde::{Deserialize, Serialize};
use warp::{
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject::{self, Rejection},
};

const BEARER: &str = "Bearer ";
const TOKEN_EXPIRATION: i64 = 60;
const JWT_SECRET: &[u8] = b"seeeecret";
#[derive(Debug, Deserialize, Clone)]
pub struct LoginInfo {
    pub password: String,
    pub email: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn verify_user(login_info: LoginInfo, db: Db) -> Option<ObjectId> {
    log::debug!("verify_user: {:?}", login_info);
    let users = db
        .client
        .unwrap()
        .database("budget_boi")
        .collection::<Document>("users");
    let user_found = users
        .find_one(
            doc! {
                "email": login_info.email,
            },
            None,
        )
        .await
        .unwrap();
    if user_found == None {
        return None;
    }
    let bson_data = user_found.unwrap();
    let user_from_doc: User = bson::from_bson(mongodb::bson::Bson::Document(bson_data)).unwrap();
    let password_hashed = user_from_doc.password_hashed;
    if password_hashed == login_info.password {
        user_from_doc.id
    }
    else {
        None
    }
}

pub fn gen_token(uid: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(TOKEN_EXPIRATION))
        .expect("valid timestamp:")
        .timestamp();
    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
}

pub async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<String, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTTokenError))?;
            Ok(decoded.claims.sub)
        }
        Err(e) => Err(reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, Error> {
    let header = headers.get(AUTHORIZATION).ok_or(Error::NoAuthHeaderError)?;
    let auth_header = std::str::from_utf8(header.as_bytes()).map_err(|_| Error::NoAuthHeaderError)?;
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
