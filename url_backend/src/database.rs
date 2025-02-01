use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

/// Type alias for a Diesel connection pool.
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Establishes a Diesel connection pool.
pub fn establish_connection() -> PgPool {
    // Load environment variables from .env file
    dotenv().ok(); 

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool")
}


/*
    Tests ======================================================================
     ________
    |__   __|      __      
      | | ___  ___| |_ ___ 
      | |/ _ \/ __| __/ __|
      | |  __/\__ \ |_\__ \
      |_|\___||___/\__|___/
    ============================================================================
*/