use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use slug;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use crate::schema::{posts, users};
use crate::models::post::{Post, PostJson};

use crate::schema::posts::dsl::{posts as all_posts};
use crate::db::DbConn;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

pub fn all(conn: &DbConn) -> Vec<Post> {
    all_posts.order(posts::id.desc()).load::<Post>(conn).unwrap()
}

pub fn create(conn: &DbConn, title: &str, body: &str) -> Post {
    let p = &NewPost {
        title,
        body
    };

    diesel::insert_into(posts::table)
        .values(p)
        .get_result::<Post>(conn)
        .expect("Error creating post")
}

fn slugify(title: &str) -> String {
    if cfg!(feature = "random-suffix") {
        format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
    } else {
        slug::slugify(title)
    }
}

fn generate_suffix(len: usize) -> String {
    let mut rng = thread_rng();
    // (0..len).map(|_| rng.sample(Alphanumeric)).collect()
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

pub fn get_post(conn: &PgConnection, slug: &str, user_id: Option<i32>) -> Option<PostJson> {
    let post = posts::table
        .filter(posts::slug.eq(slug))
        .first::<Post>(conn)
        .map_err(|err| eprintln!("articles::find_one: {}", err))
        .ok()?;

    Some(populate(conn, post))
}


fn populate(conn: &PgConnection, post: Post) -> PostJson {
    let author = users::table
        .find(post.author)
        .get_result::<User>(conn)
        .expect("Error loading author");

    post.attach(author)
}