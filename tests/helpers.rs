use rocket::local::{LocalResponse, Client};
use serde_json::Value;
use once_cell::sync::OnceCell;
use rocket::http::{Header, ContentType, Status};

/// Test user constants
pub const USERNAME: &'static str = "test_user";
pub const EMAIL: &'static str = "test_user@thefoobar.com";
pub const PASSWORD: &'static str = "test_user_password";

/// Macro for turning 'json!' in a string
#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

pub type Token = String;

/// Function for converting a response to a json value
pub fn response_json_value(response: &mut LocalResponse) -> Value {
    let body = response.body().expect("no body");
    serde_json::from_reader(body.into_inner()).expect("can't parse value")
}

/// Make an authorization header.
pub fn token_header(token: Token) -> Header<'static> {
    Header::new("authorization", format!("Token {}", token))
}

/// Function that creates a client for testing purposes
pub fn test_client() -> &'static Client {
    static INSTANCE: OnceCell<Client> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let rocket = the_foo_bar::rocket();
        Client::new(rocket).expect("valid rocket instance")
    })
}

/// Assertions for the "user" response with expected fields
pub fn check_user_response(response: &mut LocalResponse) {
    let value = response_json_value(response);
    let user = value.get("user").expect("must have a 'user' field");

    assert_eq!(user.get("email").expect("user has email"), EMAIL);
    assert_eq!(user.get("username").expect("user has username"), USERNAME);
    assert!(user.get("token").is_some());
}

/// Function to check for user validation errors
pub fn check_user_validation_errors(response: &mut LocalResponse) {
    let value = response_json_value(response);
    let username_error = value
        .get("errors")
        .expect("must have a 'errors' field")
        .get("username")
        .expect("must have 'username' errors")
        .get(0)
        .expect("must have non-empty 'username' errors")
        .as_str();

    assert_eq!(username_error, Some("has already been taken"))
}

/// Retrieve a token registering a user if required
pub fn login(client: &Client) -> Token {
    try_login(client).unwrap_or_else(|| {
        register(client, USERNAME, EMAIL, PASSWORD);
        try_login(client).expect("Cannot login")
    })
}

/// Login as default user returning None if login is not found
fn try_login(client: &Client) -> Option<Token> {
    let response = &mut client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!(
            {
                "user": {
                    "email": EMAIL,
                    "password": PASSWORD
                }
            })
        )
        .dispatch();

    if response.status() == Status::UnprocessableEntity {
        return None;
    }

    let value = response_json_value(response);
    let token = value
        .get("user")
        .and_then(|user| user.get("token"))
        .and_then(|token| token.as_str())
        .map(String::from)
        .expect("Cannot extract token");
    Some(token)
}

/// Register user
pub fn register(client: &Client, username: &str, email: &str, password: &str) {
    let response = client
        .post("/api/users")
        .header(ContentType::JSON)
        .body(json_string!(
            {"user":
                {
                    "username": username,
                    "email": email,
                    "password": password
                }
            })
        )
        .dispatch();

    match response.status() {
        Status::Ok | Status::UnprocessableEntity => {} // ok,
        status => panic!("Registration failed: {}", status)
    }
}
