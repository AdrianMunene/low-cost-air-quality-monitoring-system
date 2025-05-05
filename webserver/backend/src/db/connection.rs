use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

pub type DatabasePool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection_pool() -> DatabasePool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}
