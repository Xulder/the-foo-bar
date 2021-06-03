#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
// #[macro_use] extern crate diesel_migrations;
// #[macro_use] extern crate log;
#[macro_use] extern crate rocket_contrib;

mod models;
mod routes;
mod schema;
mod db;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;
use dotenv::dotenv;

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

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::ignite()
        .mount(
        "/api",
        routes![
            routes::posts::create_post,
            routes::posts::get_all_posts
            ]
        )
        .attach(db::DbConn::fairing())
        .attach(cors_fairing())
        .register(catchers![not_found])
}
