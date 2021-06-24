#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
// #[macro_use] extern crate diesel_migrations;
// #[macro_use] extern crate log;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;

mod auth;
mod db;
mod error;
mod models;
mod routes;
mod schema;

mod config;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;
use rocket_oauth2::OAuth2;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

#[get("/hi")]
pub fn hi() -> String {
    "Hi".to_string()
}


pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::ignite()
        .mount(
            "/api",
            routes![
                routes::posts::create_post,
                routes::posts::get_all_posts,
                // routes::posts::get_post,
                routes::users::create_user,
                routes::users::login_user,
                routes::users::get_user,
                routes::users::put_user,
                routes::oauth::github_callback,
                routes::oauth::github_login,
                hi
            ],
        )
        .attach(db::DbConn::fairing())
        .attach(cors_fairing())
        .attach(OAuth2::<routes::oauth::GitHubUserInfo>::fairing("github"))
        .attach(config::AppState::manage())
        .register(catchers![not_found])
}
