use crate::db;
use crate::db::posts::{create};
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use crate::auth::Auth;

#[derive(Deserialize)]
pub struct NewPost {
    post: NewPostData,
}

#[derive(Deserialize)]
pub struct NewPostData {
    title: String,
    body: String,
}

#[get("/posts", format = "json")]
pub fn get_all_posts(conn: db::DbConn) -> JsonValue {
    let posts = db::posts::all(&conn);

    json!({ "posts": posts })
}

/// TODO: Update this route to reflect the new post model
#[post("/posts", format = "json", data = "<new_post>")]
pub fn create_post(auth: Auth, new_post: Json<NewPost>, conn: db::DbConn) -> JsonValue {
    let new_post = new_post.into_inner().post;

    let post = create(&conn, auth.id, &new_post.title, &new_post.body);

    json!({ "post": post })
}

#[get("/posts/<slug>")]
pub fn get_post(slug: String, conn: db::DbConn) -> Option<JsonValue> {
    db::posts::get_post(&conn, &slug).map(|post| json!({ "post": post }))
}
