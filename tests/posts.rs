mod helpers;

use helpers::*;
use rocket::http::{ContentType, Status};
use rocket::local::{LocalResponse, Client};

const POST_TITLE: &str = "Test post";
const POST_BODY: &str = "This is a test post!";

#[test]
/// TODO: Update this test to reflect the new post model
fn test_create_post() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_post(&client, token);

    let value = response_json_value(response);

    let title = value
        .get("post")
        .expect("must have a 'post' field")
        .get("title")
        .expect("must have a 'title' field")
        .as_str();

    let body = value
        .get("post")
        .expect("must have a 'post' field")
        .get("body")
        .expect("must have a 'body' field")
        .as_str();

    assert_eq!(title, Some(POST_TITLE));
    assert_eq!(body, Some(POST_BODY));
}

fn create_post(client: &Client, token: Token) -> LocalResponse {
    let response = client
        .post("/api/posts")
        .header(ContentType::JSON)
        .header(token_header(token))
        .body(json_string!({
                "post": {
                    "title": POST_TITLE,
                    "body": POST_BODY,
                }
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    response
}
