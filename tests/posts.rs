mod helpers;

use helpers::*;
use rocket::http::ContentType;

const POST_TITLE: &str = "Test post";
const POST_BODY: &str = "This is a test post!";

#[test]
fn test_create_post() {
    let client = test_client();
    let response = &mut client
        .post(format!("/api/posts"))
        .header(ContentType::JSON)
        .body(json_string!({
            "post": {
                "title": "Test post",
                "body": "This is a test post!",
            }
        }))
        .dispatch();

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
