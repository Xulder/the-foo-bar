use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::posts;
use crate::models::post::{Post};

use crate::schema::posts::dsl::{posts as all_posts};

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

pub fn all(conn: &PgConnection) -> Vec<Post> {
    all_posts.order(posts::id.desc()).load::<Post>(conn).unwrap()
}

pub fn create(conn: &PgConnection, title: &str, body: &str) -> Post {
    let p = &NewPost {
        title,
        body
    };

    diesel::insert_into(posts::table)
        .values(p)
        .get_result::<Post>(conn)
        .expect("Error creating post")
}
