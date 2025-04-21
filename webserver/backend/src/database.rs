use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ self, ConnectionManager };
use dotenvy::dotenv;
use std::env;
use std::time::Duration;

pub type DatabasePool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection_pool() -> DatabasePool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(5))
        .connection_customizer(Box::new(ConnectionOptions))
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool");

    // Test the connection pool immediately
    pool.get().expect("Failed to verify database connection on startup");

    println!("Database connection pool successfully established");
    
    pool
}

#[derive(Debug)]
struct ConnectionOptions;

impl r2d2::CustomizeConnection<SqliteConnection, r2d2::Error> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), r2d2::Error> {
        use diesel::Connection;
        conn.begin_test_transaction()
            .map_err(|e| r2d2::Error::QueryError(e))?;
        Ok(())
    }
}
