#![allow(unused_doc_comments)]

/// This file contains the starts the Meals Service which contains the Meals and Dishes APIs
/// The main function initializes the logger, creates a connection pool to the database,
/// and starts the Actix web server on the defined port and host

mod db;
mod meals;
mod schema;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use env_logger;
use db::{create_pool, DbPool};
use meals::*;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 8000;

/// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    ///Initialize the logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    /// Create a connection pool to the database
    let pool: DbPool = create_pool().expect("Failed to create pool");

    /// Start the Actix web server and bind it to port 8080
    ///
    /// The server is configured to use the routes defined in the routes module
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(index)
            .service(collection_deletion)
            .service(meals_collection_deletion)
            .service(get_all_dishes)
            .service(create_dish)
            .service(get_dish)
            .service(get_dish_by_name)
            .service(delete_dish)
            .service(delete_dish_by_name)
            .service(get_all_meals)
            .service(create_meal)
            .service(get_meal)
            .service(get_meal_by_name)
            .service(delete_meal)
            .service(delete_meal_by_name)
            .service(update_meal)
    })
        .bind((HOST, PORT))?
        .run()
        .await


}