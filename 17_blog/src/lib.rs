use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub type PoolConnection =
    diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

fn get_connection_string() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_connection() -> PgConnection {
    let database_url = get_connection_string();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_pool_connection() -> PoolConnection {
    let database_url = get_connection_string();
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(connection)
        .expect("Failed to create pool")
}
