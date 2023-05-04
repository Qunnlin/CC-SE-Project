#![allow(unused_doc_comments)]
use actix_web::{App, HttpServer};
use dishes_routes::{get_all_dishes, index, create_dish, collection_deletion, get_dish, get_dish_by_name, delete_dish, delete_dish_by_name};
use meals_routes::{meals_collection_deletion, get_all_meals, create_meal, get_meal, get_meal_by_name, delete_meal, delete_meal_by_name, update_meal};
mod models;
mod ninjas_api;
mod db;
mod dishes_routes;
mod schema;
mod meals_routes;


/// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    /// Start the Actix web server and bind it to port 8080
    ///
    /// The server is configured to use the routes defined in the routes module
    HttpServer::new(|| {
        App::new()
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
        .bind(("0.0.0.0", 8000))?
        .run()
        .await


}