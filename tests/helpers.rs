use rocket::local::{LocalResponse, Client};
use serde_json::Value;
use once_cell::sync::OnceCell;

// Macro for turning 'json!' in a string
#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

// Function for converting a response to a json value
pub fn response_json_value(response: &mut LocalResponse) -> Value {
    let body = response.body().expect("no body");
    serde_json::from_reader(body.into_inner()).expect("can't parse value")
}

// Function that creates a client for testing purposes
pub fn test_client() -> &'static Client {
    static INSTANCE: OnceCell<Client> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let rocket = the_foo_bar::rocket();
        Client::new(rocket).expect("valid rocket instance")
    })
}
