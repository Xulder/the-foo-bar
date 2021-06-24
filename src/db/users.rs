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
use serde::Deserialize;

use crate::schema::users::dsl::{users as all_users};
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
    conn: &PgConnection,
    username: &str,
    email: &str,
    password: &str
) -> Result<User, UserCreationError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = &argon2
        .hash_password_simple(password.as_ref(), salt.as_ref())
        .expect("hash failed")
        .to_string();

    let new_user = &NewUser {
        username,
        email,
        hash: password_hash,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Into::into)
}

pub fn get_user(
    conn: &PgConnection,
    id: i32
) -> Option<User> {
    all_users
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

pub fn login(
    conn: &PgConnection,
    email: &str,
    password: &str
) -> Option<User> {
    let user = all_users // this should be Result<User> can use debug on clion
        .filter(users::email.eq(email))
        .get_result::<User>(conn)
        .map_err(|err| eprintln!("login user: {}", err))
        .ok()?;

    let argon2 = Argon2::default();

    let parsed_pwd = PasswordHash::new(&user.hash).unwrap();

    let password_matches = argon2
        .verify_password(password.as_ref(), &parsed_pwd)
        .is_ok();

    if password_matches {
        Some(user)
    } else {
        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            email
        );
        None
    }
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "users"]
pub struct UpdateUserData {
    username: Option<String>,
    email: Option<String>,

    #[column_name = "hash"]
    password: Option<String>
}

pub fn update_user(
    conn: &PgConnection,
    id: i32,
    data: &UpdateUserData
) -> Option<User> {
    let data = &UpdateUserData {
        password: None,
        ..data.clone()
    };

    diesel::update(users::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}

