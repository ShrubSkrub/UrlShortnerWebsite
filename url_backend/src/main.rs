use database::establish_connection;

mod database;
mod models;

fn main() {
    let pool = establish_connection();

    // Get a connection from the pool
    let conn = pool.get().expect("Failed to get DB connection");

    println!("Connected to the database!");
}
