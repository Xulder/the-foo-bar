// use crate::models::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::posts;
use crate::models::post::Post;
// use crate::models::post::PostJson

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

//TODO: figure out why this doesn't work with return PostJson
pub fn create(conn: &PgConnection, title: &str, body: &str) -> Post {
    // Todo: create a new post in the table
    let p = &NewPost {
        title,
        body
    };

    diesel::insert_into(posts::table).values(p).get_result::<Post>(conn).expect("Error creating post")
}
