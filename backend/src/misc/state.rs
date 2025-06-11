use std::sync::Arc;

use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};

#[derive(Clone)]
pub struct GlobalState {
    pub db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl GlobalState {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let db_pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Failed to create database connection pool");

        GlobalState {
            db_pool: Arc::new(db_pool),
        }
    }

    pub fn get_db_conn(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
        self.db_pool.get()
    }
}
