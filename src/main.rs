use dotenv::dotenv;
use std::env;
use routes::*;


mod db;
mod models;
mod routes;
mod schema;
mod ninjas_api;




fn main() {

    use self
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = &mut db::establish_connection(database_url);


}