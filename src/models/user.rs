use serde::Serialize;
use validator::{Validate, ValidationError};
use chrono::{Duration, Utc};
use crate::auth::Auth;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    username: &'a str,
    email: &'a str,
    token: String
}

impl User {
    pub fn to_user_auth(&self, secret: &[u8]) -> UserAuth {
        let exp = Utc::now() + Duration::days(30);
        let token = Auth {
            id: self.id,
            username: self.username.clone(),
            exp: exp.timestamp(),
        }
        .token(secret);

        UserAuth {
            username: &self.username,
            email: &self.email,
            token,
        }
    }
}