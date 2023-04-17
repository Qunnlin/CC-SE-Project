use actix_web::{guard, App, HttpServer, web};
use routes::{get_all_dishes, index, create_dish, collection_deletion, get_dish, get_dish_by_name, delete_dish, delete_dish_by_name};

mod models;
mod ninjas_api;
mod routes;
mod schema;


/// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    /// Start the Actix web server and bind it to port 8080
    ///
    /// The server is configured to use the routes defined in the routes module
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_all_dishes)
            .service(create_dish)
            .service(collection_deletion)
            .service(get_dish)
            .service(get_dish_by_name)
            .service(delete_dish)
            .service(delete_dish_by_name)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await


}