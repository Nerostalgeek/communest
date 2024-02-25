use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use std::env;
use std::sync::Arc;
use tokio::task;

use crate::services::user_service::UserServiceError;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_database_pool() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub async fn get_db_connection(
    pool: Arc<DbPool>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, UserServiceError> {
    let pool_result = task::spawn_blocking(move || pool.get()).await;
    match pool_result {
        Ok(Ok(connection)) => Ok(connection),
        Ok(Err(_)) | Err(_) => Err(UserServiceError::DatabaseConnectionPoolError),
    }
}
