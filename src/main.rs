#![feature(proc_macro_hygiene, decl_macro)]

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![ping])
        .launch();
}
