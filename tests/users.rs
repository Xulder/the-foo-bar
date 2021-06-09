mod helpers;

use helpers::*;
use rocket::http::{ContentType, Status};
use rocket::local::LocalResponse;

#[test]
/// Register user and check for correct status
fn test_register_user() {
    let client = test_client();
    let response = &mut client
        .post("/api/users")
        .header(ContentType::JSON)
        .body(json_string!(
            {
                "user": {
                    "username": USERNAME,
                    "email": EMAIL,
                    "password": PASSWORD,
                }
            }
        ))
        .dispatch();

    let status = response.status();

    // If user was created already, we get UnprocessableEntity. Otherwise we get Ok.
    match status {
        Status::Ok => check_user_response(response),
        Status::UnprocessableEntity => check_user_validation_errors(response),
        _ => panic!("Got status: {}", status)
    }
}

#[test]
/// Registration with an already taken username must fail
fn test_register_with_duplicated_username() {
    let client = test_client();
    register(client, "duplicate", "duplicate@thefoobar.com", PASSWORD);

    let response = &mut client
        .post("/api/users")
        .header(ContentType::JSON)
        .body(json_string!({
            "user": {
                "username": "duplicate",
                "email": "duplicate@thefoobar.com",
                "password": PASSWORD,
            },
        }))
        .dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);

    let value = response_json_value(response);
    let error = value
        .get("errors")
        .and_then(|errors| errors.get("username"))
        .and_then(|errors| errors.get(0))
        .and_then(|error| error.as_str());

    assert_eq!(error, Some("has already been taken"))
}

#[test]
/// Check that `/user` endpoint returns the correct data
fn test_get_user() {
    let client = test_client();
    let token = login(&client);
    let response = &mut client
        .get("/api/users")
        .header(token_header(token))
        .dispatch();

    check_user_response(response);
}

#[test]
/// Login a user and check for a token
fn test_login() {
    let client = test_client();
    let response = &mut client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!({
            "user": {
                "email": EMAIL,
                "password": PASSWORD
            }
        }))
        .dispatch();

    let value = response_json_value(response);
    value
        .get("user")
        .expect("must have a 'user' field")
        .get("token")
        .expect("user has token")
        .as_str()
        .expect("token must be a string");
}

#[test]
/// Check if login fails with wrong password
fn test_incorrect_login() {
    let client = test_client();
    let response = &mut client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!({
            "user": {
                "email": EMAIL,
                "password": "wrong"
            }
        }))
        .dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);

    let value = response_json_value(response);
    let login_error = value
        .get("errors")
        .expect("must have a 'errors' field")
        .get("email or password")
        .expect("must have 'email or password' errors")
        .get(0)
        .expect("must have non empty 'email or password' errors")
        .as_str();

    assert_eq!(login_error, Some("is invalid"));
}
