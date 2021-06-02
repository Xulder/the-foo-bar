use crate::models::*;
use crate::schema::posts;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

pub fn create(conn: &PgConnection, title: String, body: String) {
    // Todo: create a new post in the table
}
