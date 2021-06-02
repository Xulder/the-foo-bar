use diesel::{Queryable, Insertable};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostJson {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}