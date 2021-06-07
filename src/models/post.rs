use diesel::{Queryable};
use serde::Serialize;
use crate::models::user::User;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub author: i32,
    pub published: bool,
}

impl Post {
    pub fn attach(self, author: User) -> PostJson {
        PostJson {
            id: self.id,
            slug: self.slug,
            title: self.title,
            body: self.body,
            author,
            published: self.published
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostJson {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub author: User,
    pub published: bool,
}
