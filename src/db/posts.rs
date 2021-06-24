use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use slug;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use crate::schema::{posts, users};
use crate::models::post::{Post, PostJson};

use crate::schema::posts::dsl::{posts as all_posts};
use core::iter;
use crate::models::user::User;

const SUFFIX_LEN: usize = 6;
const DEFAULT_LIMIT: i64 = 20;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
    pub author: i32,
}

pub fn all(conn: &PgConnection) -> Vec<Post> {
    all_posts.order(posts::id.desc()).load::<Post>(conn).unwrap()
}

/// TODO: Update this function to reflect the new post model
pub fn create(conn: &PgConnection, author: i32, title: &str, body: &str) -> PostJson {
    let new_post = &NewPost {
        title,
        body,
        author,
        slug: &slugify(title),
    };

    let author = users::table
        .find(author)
        .get_result::<User>(conn)
        .expect("Error loading author");

    diesel::insert_into(posts::table)
        .values(new_post)
        .get_result::<Post>(conn)
        .expect("Error creating post")
        .attach(author)
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

#[derive(FromForm, Default)]
pub struct FeedPosts {
    limit: Option<i64>,
    offset: Option<i64>,
}

// pub fn feed(conn: &PgConnection, params: &FeedArticles) {
//
// }

pub fn get_post(conn: &PgConnection, slug: &str) -> Option<PostJson> {
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
