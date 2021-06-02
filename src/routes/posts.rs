use crate::db;
use crate::db::posts::{create};
// use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewPost {
    post: NewPostData,
}

#[derive(Deserialize)]
pub struct NewPostData {
    title: String,
    body: String,
}

#[post("/posts", format = "json", data = "<new_post>")]
pub fn create_post(new_post: Json<NewPost>, conn: db::DbConn) -> JsonValue {
    let new_post = new_post.into_inner().post;

    let post = create(&conn, &new_post.title, &*new_post.body);

    json!({ "post": post })
}
