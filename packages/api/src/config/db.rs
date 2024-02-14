use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_database_pool(database_url: &str) -> DbPool {
    dotenv().ok();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
