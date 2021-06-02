pub mod posts;

#[database("diesel_postgres_pool")]
pub struct DbConn(diesel::PgConnection);
