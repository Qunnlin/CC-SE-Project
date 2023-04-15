use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use diesel::{Connection, QueryResult};

// Create a struct to hold the DB connection
pub struct DbConn(pub PgConnection);

// Create a function to get the DB connection
fn init_db_connection(db_url: String) -> DbConn {
    DbConn(PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url)))
}

impl DbConn {
    // Create a function to get the DB connection from the environment
    pub fn establish_connection() -> DbConn {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        init_db_connection(database_url)
    }

    // Create a function to get the DB connection from a string
    pub fn establish_connection_from_string(db_url: String) -> DbConn {
        init_db_connection(db_url)
    }

}



