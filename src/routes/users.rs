use crate::{db, db::users::create};
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use crate::error::{FieldValidator, Errors};
use crate::config::AppState;
use rocket::State;
use validator::{Validate, ValidationError};
use crate::db::users::UserCreationError;

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[derive(Validate, Deserialize)]
struct NewUserData {
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}

//
// fn validate_username(username: &str) -> Result<(), ValidationError> {
//     // TODO: Query database for matching names
//     if username == "notcool" {
//         return Err(ValidationError::new("Rip, b"));
//     }
//
//     Ok(())
// }

#[post("/users", format = "json", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUser>, conn: db::DbConn, state: State<AppState>) -> Result<JsonValue, Errors> {
    let new_user = new_user.into_inner().user;
    let mut extractor = FieldValidator::validate(&new_user);
    let username = extractor.extract("username", new_user.username);
    let usertag = extractor.extract("usertag", new_user.usertag);
    let email = extractor.extract("email", new_user.email);
    let password = extractor.extract("password", new_user.password);

    extractor.check()?;

    db::users::create(&conn, &username, &usertag, &email, &password)
        .map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
        .map_err(|error| {
            let field = match error {
                UserCreationError::DuplicatedEmail => "email",
                UserCreationError::DuplicatedUsername => "username",
            };
            Errors::new(&[(field, "has already been taken")])
        })
}