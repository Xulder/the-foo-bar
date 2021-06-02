#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_cors;

use dotenv::dotenv;

mod models;
mod routes;
mod schema;
mod db;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::ignite().mount(
                        "/api",
                        routes![
                            routes::posts::create,
                            routes::posts::
                        ]
    )
}