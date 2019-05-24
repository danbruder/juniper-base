//
// db.rs
//

use diesel::pg::PgConnection;
use diesel::r2d2;
use dotenv::dotenv;
use std::env;

pub type ConnectionManager = r2d2::ConnectionManager<PgConnection>;
pub type ConnectionPool = r2d2::Pool<ConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

pub struct Db { 
    pub pool: ConnectionPool,
}

pub fn create_pool() -> ConnectionPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

