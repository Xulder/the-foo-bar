// TODO: Write test for post creation
// use rocket::local::{Client};
// use the_foo_bar;
// use rocket::local::LocalResponse;
// use serde_json::Value;
// use rocket::http::ContentType;
//
// #[macro_export]
// macro_rules! json_string {
//     ($value:tt) => {
//         serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
//     };
// }
//
// pub fn response_json_value(response: &mut LocalResponse) -> Value {
//     let body = response.body().expect("no body");
//     serde_json::from_reader(body.into_inner()).expect("can't parse value")
// }
//
//
// const POST_TITLE: &str = "Test post";
// //const POST_BODY: &str = "This is a test post!";
//
// #[test]
// fn test_create_post() {
//     let rocket = the_foo_bar::rocket();
//     let client = Client::new(rocket).expect("valid rocket instance");
//     let response = &mut client
//         .post(format!("/api/posts"))
//         .header(ContentType::JSON)
//         .body(json_string!({
//             "post": {
//                 "description": "test",
//             }
//         }))
//         .dispatch();
//
//     let value = response_json_value(response);
//     let title = value
//         .get("post")
//         .expect("must have a 'post' field")
//         .get("title")
//         .expect("must have a 'title' field")
//         .as_str();
//
//     assert_eq!(title, Some(POST_TITLE));
// }
