use diesel::insert_into;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod ninjas_api;
mod routes;
mod schema;

use crate::models::NewDish;
use crate::schema::dishes::dsl::dishes;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() {

    let connection = &mut establish_connection();
    let dish_name = "chicken";
    let nutrition_info = ninjas_api::get_nutrition_info(dish_name).await;

    let new_dish = NewDish {
        name: dish_name.to_string(),
        calories: nutrition_info[0].calories,
        sodium: nutrition_info[0].sodium_mg,
        sugar: nutrition_info[0].sugar_g,
        serving_size: nutrition_info[0].serving_size_g,
    };


    let result = insert_into(dishes).values(&new_dish).execute(connection);

    println!("{:?}", result);





}