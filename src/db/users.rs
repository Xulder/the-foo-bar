use crate::schema::users;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use rand_core::OsRng;
use validator::ValidationError;

use crate::schema::users::dsl::users as all_users;
use crate::db::DbConn;
use crate::models::user::User;

///
/// # Users
///
///
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    //pub usertag: &'a [u8; 4],
    pub email: &'a str,
    pub hash: &'a str,
}

pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => return UserCreationError::DuplicatedUsername,
                Some("users_email_key") => return UserCreationError::DuplicatedEmail,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

pub fn create(
    conn: &DbConn,
    username: &str,
    usertag: &[u8; 4],
    email: &str,
    password: &str
) -> User {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = &argon2
        .hash_password_simple(password.as_ref(), salt.as_ref())
        .unwrap()
        .to_string();

    let new_user = &NewUser {
        username,
        email,
        usertag,
        hash: password_hash,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Into::into)
}
