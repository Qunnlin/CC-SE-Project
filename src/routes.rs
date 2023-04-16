use std::env;
use super::models::{Dish, NewDish, ReqDish};
use serde_json::{from_slice, json};
use super::ninjas_api::get_nutrition_info;
use actix_web::{get, post, put, delete, HttpResponse, Responder, HttpRequest, FromRequest, web, error};
use actix_web::web::Json;
use actix_web::web::Payload;
use actix_web::guard::Delete;
use diesel;
use diesel::prelude::*;
use diesel::{Connection, insert_into, PgConnection, QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use futures::StreamExt;
use crate::schema::dishes::dsl::dishes;
use crate::schema::dishes::{dish_id, name};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Welcome to the Dish API"
    }))
}

// Get all dishes from the database
#[get("/dishes")]
pub async fn  get_all_dishes() -> impl Responder {
    let conn = &mut establish_connection();
    let all_dishes = dishes.load::<Dish>(conn);

    let all_dishes = match all_dishes {
        Ok(all_dishes) => all_dishes,
        Err(e) => panic!("Error: {}", e),
    };

    HttpResponse::Ok().json(all_dishes)


}

// Add a new dish to the database
#[post("/dishes")]
pub async fn create_dish(request: HttpRequest, mut payload: Payload) -> impl Responder {

    // Check if the request is application/json
    match request.headers().get("Content-Type") {
        Some(content_type) => {
            if content_type != "application/json" {
                return HttpResponse::UnsupportedMediaType().json(json!({
                    "message": "Content-Type must be application/json",
                    "id": "0"
                }))
            }
        },
        None => {
            return HttpResponse::UnsupportedMediaType().json(json!({
                "message": "Content-Type must be application/json",
                "id": "0"
            }))
        }
    }

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > 262_144 {
            return HttpResponse::PayloadTooLarge().json(json!({
                "message": "Payload too large",
                "id": "-6"
            }))
        }
        body.extend_from_slice(&chunk);
    }

    let body = from_slice::<ReqDish>(&body);
    let body = match body {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::UnsupportedMediaType().json(json!({
                "message": "Name is required",
                "id": "-1"
            }))
        }
    };


    // Get the nutrition info from the Ninjas API
    let nut_info = get_nutrition_info(&*body.name).await;
    let nut_info = match nut_info {
        Ok(nut_info) => {
            if nut_info.is_empty() {
                return HttpResponse::UnprocessableEntity().json(json!({
                    "message": "Dish not found in Ninjas API",
                    "id": "-3"
                }))
            } else {
                nut_info[0].clone()
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::GatewayTimeout().json(json!({
                "message": "Ninjas API not responding",
                "id": "-4"

            }))
        }
    };

    // Get the database connection
    let conn = &mut establish_connection();

    // Get the first element of the vector
    let new_dish = NewDish {
        name: body.name.clone(),
        calories: nut_info.calories,
        sodium: nut_info.sodium_mg,
        sugar: nut_info.sugar_g,
        serving_size: nut_info.serving_size_g,
    };

    // Try to insert the dish into the database
    let new_dish = insert_into(dishes).values(new_dish).execute(conn);

    // Check if the dish already exists
    match new_dish {
        Ok(new_dish) => new_dish,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::UnprocessableEntity().json(json!({
                "message": "Dish already exists",
                "id": "-2"
            }))

        }
    };

    // Get the dish id, not nice but it works
    let new_dish_id = dishes.filter(name.eq(&*body.name)).select(dish_id).first::<i32>(conn);
    let new_dish_id = match new_dish_id {
        Ok(new_dish_id) => new_dish_id,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message": "Error getting dish id",
                "id": "-5"
            }))
        }
    };

    // Return the dish id
    HttpResponse::Created().json(json!({
        "message": "Dish created successfully",
        "id": new_dish_id,
    }))
}

#[delete("/dishes")]
pub async fn collection_deletion() -> impl Responder {
    HttpResponse::MethodNotAllowed().json(json!({
        "message": "Method not allowed",
        "id": "-7"
    }))
}


