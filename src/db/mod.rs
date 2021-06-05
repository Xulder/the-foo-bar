use rocket_contrib::databases::diesel;

pub mod posts;
pub mod users;

#[database("diesel_postgres_pool")]
pub struct DbConn(diesel::PgConnection);
