use actix_web::{App, HttpServer};
use routes::{get_all_dishes, index, create_dish};
use crate::routes::collection_deletion;

mod models;
mod ninjas_api;
mod routes;
mod schema;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_all_dishes)
            .service(create_dish)
            .service(collection_deletion)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await


}