#![allow(unused_doc_comments)]

use diesel::pg::PgConnection;

use dotenv::dotenv;
use std::env;
use diesel::{Connection};

///
/// Establishes a connection to the database
///
/// Returns a [PgConnection]
pub fn establish_connection() -> PgConnection {
    /// Load the .env file
    dotenv().ok();
    /// Get the database URL from the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    /// Establish a connection to the database
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


