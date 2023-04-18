use super::models::{Dish, NewDish, ReqDish};
use serde_json::{from_slice, json};
use super::ninjas_api::get_nutrition_info;
use super::db::establish_connection;
use actix_web::{get, post, delete, HttpResponse, Responder, HttpRequest, web};

use actix_web::web::Payload;

use diesel;
use diesel::prelude::*;
use diesel::{insert_into, QueryDsl, RunQueryDsl};

use futures::StreamExt;
use crate::schema::dishes::dsl::dishes;
use crate::schema::dishes::{dish_id, name};

/// Error codes as defined in the Assignment
const NOT_JSON: &str = "0";
const PARAM_NOT_FOUND: &str = "-1";
const DISH_ALREADY_EXISTS: &str = "-2";
const DISH_NOT_RECOGNIZED: &str = "-3";
const NINJAS_UNAVAILABLE: &str = "-4";
const DISH_NOT_FOUND: &str = "-5";

///
/// Creates the default route for the API in "/"
///
/// Returns a [HttpResponse::Ok] with a JSON body
#[get("/")]
pub async fn index() -> impl Responder {
    /// Return a JSON response with a message
    HttpResponse::Ok().json(json!({
        "message": "Welcome to the MEALS API"
    }))
}

/*
=============================== GET /dishes ===============================
 */

///
/// Creates the route for getting all dishes in "/dishes"
///
/// Returns a [HttpResponse::Ok] with a JSON body
#[get("/dishes")]
pub async fn  get_all_dishes() -> impl Responder {
    /// Get a connection to the database
    let conn = &mut establish_connection();
    /// Load all dishes from the database
    let all_dishes = dishes.load::<Dish>(conn);
    /// Check if the query was successful
    let all_dishes = match all_dishes {
        Ok(all_dishes) => all_dishes,
        Err(e) => panic!("Error: {}", e),
    };

    /// Return a JSON response with all the dishes
    HttpResponse::Ok().json(all_dishes)
}

/*
=============================== POST /dishes ===============================
 */

///
/// Creates the route for creating a dish in "/dishes"
///
/// Returns a [HttpResponse::Created] with a JSON body containing the ID of the new dish
#[post("/dishes")]
pub async fn create_dish(request: HttpRequest, mut payload: Payload) -> impl Responder {

    /// Check if the Content-Type is application/json
    ///
    /// If it is not, return a [HttpResponse::UnsupportedMediaType] with a Error Code 0
    match request.headers().get("Content-Type") {
        Some(content_type) => {
            if content_type != "application/json" {
                return HttpResponse::UnsupportedMediaType().body(NOT_JSON)
            }
        },
        None => {
            return HttpResponse::UnsupportedMediaType().body(NOT_JSON)
        }
    }

    /// Read the body of the request
    ///
    /// If the body is too large, return a [HttpResponse::PayloadTooLarge]
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > 262_144 {
            return HttpResponse::PayloadTooLarge().json(json!({
                "message": "Payload too large",
            }))
        }
        body.extend_from_slice(&chunk);
    }

    /// Deserialize the body into a [ReqDish] struct
    let body = from_slice::<ReqDish>(&body);
    /// Check if the deserialization was successful and the required fields are present
    ///
    /// If it was not, return a [HttpResponse::UnsupportedMediaType] with a Error Code -1
    let body = match body {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::UnsupportedMediaType().body(PARAM_NOT_FOUND)
        }
    };

    /// Get the nutrition information from the Ninjas API
    ///
    /// If the API is not responding, return a [HttpResponse::GatewayTimeout] with a Error Code -4
    /// If the dish is not found, return a [HttpResponse::UnprocessableEntity] with a Error Code -3
    let nut_info = get_nutrition_info(&*body.name).await;
    let nut_info = match nut_info {
        Ok(nut_info) => {
            if nut_info.is_empty() {
                return HttpResponse::UnprocessableEntity().body(DISH_NOT_RECOGNIZED)
            } else {
                nut_info[0].clone()
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::GatewayTimeout().body(NINJAS_UNAVAILABLE)
        }
    };

    /// Get a connection to the database
    let conn = &mut establish_connection();

    /// Create a new dish struct with the nutrition information
    let new_dish = NewDish {
        name: body.name.clone(),
        calories: nut_info.calories,
        sodium: nut_info.sodium_mg,
        sugar: nut_info.sugar_g,
        serving_size: nut_info.serving_size_g,
    };

    /// Insert the new dish into the database
    let new_dish = insert_into(dishes).values(new_dish).execute(conn);

    /// Check if the insertion was successful
    ///
    /// If it was not, return a [HttpResponse::UnprocessableEntity] with a Error Code -2
    match new_dish {
        Ok(new_dish) => new_dish,
        Err(e) => {
            return HttpResponse::UnprocessableEntity().body(DISH_ALREADY_EXISTS)
        }
    };

    /// Get the ID of the newly inserted dish
    ///
    /// If retrieving the ID fails, return a [HttpResponse::InternalServerError] with a Error Code
    /// TODO: Find a better way to get the ID of the newly inserted dish
    let new_dish_id = dishes.filter(name.eq(&*body.name)).select(dish_id).first::<i32>(conn);
    let new_dish_id = match new_dish_id {
        Ok(new_dish_id) => new_dish_id,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message": "Error getting dish id",
            }))
        }
    };

    /// Return a [HttpResponse::Created] with a JSON body containing the ID of the new dish
    HttpResponse::Created().body(new_dish_id.to_string())
}

/*
=============================== DELETE /dishes ===============================
 */
///
/// Creates the route for deleting a dish in "/dishes"
///
/// Returns a [HttpResponse::MethodNotAllowed] with a JSON body containing an error message and the error code -7
#[delete("/dishes")]
pub async fn collection_deletion() -> impl Responder {

    /// Return a [HttpResponse::MethodNotAllowed] with a JSON body containing an error message and the error code -7
    HttpResponse::MethodNotAllowed()
}

/*
=============================== GET /dishes/{id} ===============================
 */

/// Creates the route for getting a dish by id in "/dishes/{id}"
/// ## Arguments
/// * `id` - The name of the dish to be retrieved
/// ## Returns
/// * [HttpResponse::Ok] with a JSON body containing the dish
#[get("/dishes/{id:\\d+}")]
pub async fn get_dish(id: web::Path<i32>) -> impl Responder {

    /// Get a connection to the database
    let conn = &mut establish_connection();

    /// Get the dish from the database
    let dish = dishes.find(&*id).first::<Dish>(conn);

    /// Check if the dish was found in the database
    ///
    /// If it was not, return a [HttpResponse::NotFound] with a JSON body containing an error message and the error code -5
    let dish = match dish {
        Ok(dish) => dish,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(DISH_NOT_FOUND)
        }
    };

    /// Return a [HttpResponse::Ok] with a JSON body containing the dish
    HttpResponse::Ok().json(dish)
}

/*
=============================== DELETE /dishes/{id} ===============================
 */

/// Creates the route for deleting a dish by id in "/dishes/{id}"
/// ## Arguments
/// * `id` - The name of the dish to be deleted
/// ## Returns
/// * [HttpResponse::Ok] with a JSON body containing the dish
#[delete("/dishes/{id:\\d+}")]
pub async fn delete_dish(id: web::Path<i32>) -> impl Responder {
    /// Get a connection to the database
    let conn = &mut establish_connection();

    /// Get the dish from the database
    let dish = dishes.find(&*id).first::<Dish>(conn);

    /// Check if the dish was found in the database
    /// If it was not, return a [HttpResponse::NotFound] with a JSON body containing an error message and the error code -5
    match dish {
        Ok(dish) => dish,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(DISH_NOT_FOUND)
        }
    };

    /// Delete the dish from the database
    let delete_dish = diesel::delete(dishes.find(&*id)).execute(conn);

    /// Check if the dish was deleted from the database
    ///
    /// If it was not, return a [HttpResponse::InternalServerError] with a JSON body containing an error message and the error code -8
    match delete_dish {
        Ok(delete_dish) => delete_dish,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message": "Error deleting dish",
            }))
        }
    };

    /// Return a [HttpResponse::Ok] with a JSON body containing a success message and the id of the deleted dish
    HttpResponse::Ok().body(id.into_inner().to_string())
}

/*
=============================== GET /dishes/{name} ===============================
 */

/// Creates the route for getting a dish by name in "/dishes/{name}"
/// ## Arguments
/// * `dish_name` - The name of the dish to be retrieved
/// ## Returns
/// * [HttpResponse::Ok] with a JSON body containing the dish
#[get("/dishes/{name:.*}")]
pub async fn get_dish_by_name(dish_name: web::Path<String>) -> impl Responder {

    /// Get a connection to the database
    let conn = &mut establish_connection();

    /// Get the dish from the database
    let dish = dishes.filter(name.eq(&*dish_name)).first::<Dish>(conn);

    /// Check if the dish was found in the database
    ///
    /// If it was not, return a [HttpResponse::NotFound] with a JSON body containing an error message and the error code -5
    let dish = match dish {
        Ok(dish) => dish,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(DISH_NOT_FOUND)
        }
    };

    /// Return a [HttpResponse::Ok] with a JSON body containing the dish
    HttpResponse::Ok().json(dish)
}

/*
=============================== DELETE /dishes/{name} ===============================
 */

/// Creates the route for deleting a dish by name in "/dishes/{name}"
/// ## Arguments
/// * `dish_name` - The name of the dish to be deleted
/// ## Returns
/// * [HttpResponse::Ok] with a JSON body containing the dish
#[delete("/dishes/{name:.*}")]
pub async fn delete_dish_by_name(dish_name: web::Path<String>) -> impl Responder {

    /// Get a connection to the database
    let conn = &mut establish_connection();

    /// Check if the dish exists in the database and get its ID
    let id = dishes.filter(name.eq(&*dish_name)).select(dish_id).first::<i32>(conn);

    /// Check if the ID could be retrieved
    ///
    /// If it was not, return a [HttpResponse::NotFound] with a JSON body containing an error message and the error code -5
    let id = match id {
        Ok(new_dish_id) => new_dish_id,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(DISH_NOT_FOUND)
        }
    };

    /// Delete the dish from the database
    let deletion_result = diesel::delete(dishes.filter(name.eq(&*dish_name))).execute(conn);

    /// Check if the dish was deleted from the database
    ///
    /// If it was not, return a [HttpResponse::InternalServerError] with a JSON body containing an error message and the error code -8
    match deletion_result {
        Ok(deletion_result) => deletion_result,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message": "Error deleting dish",
            }))
        }
    };

    /// Return a [HttpResponse::Ok] with a JSON body containing a success message and the id of the deleted dish
    HttpResponse::Ok().body(id.to_string())
}







