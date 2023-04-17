use crate::schema::meals::dsl::*;
use super::models::{Meal, NewMeal};
use super::db::establish_connection;

use actix_web::{get, post, delete, HttpResponse, Responder, HttpRequest, web};
use actix_web::web::Payload;

use diesel;
use diesel::prelude::*;
use diesel::{insert_into, QueryDsl, RunQueryDsl};
use futures::StreamExt;

use serde::{Serialize, Deserialize};
use serde_json::{from_slice, json};
use crate::models::Dish;
use crate::schema::dishes::dsl::dishes;


/*
=============================== GET /meals ===============================
 */
/// Creates the route for getting all meals in "/meals"
///
/// Returns a  [HttpResponse] with a status of 200 and a JSON body containing all meals
#[get("/meals")]
pub async fn get_all_meals() -> impl Responder {
    /// Establish a connection to the database
    let conn = &mut establish_connection();
    /// Get all meals from the database
    let results = meals.load::<Meal>(conn).expect("Error loading meals");
    /// Return a 200 response with the meals in the body
    HttpResponse::Ok().json(results)
}

/*
=============================== POST /meals ===============================
 */
/// Creates the route for creating a meal in "/meals"
///
/// Creates a new meal in the database, based on the JSON body of the request
///
/// Returns a [HttpResponse] with a status of 201 and a JSON body containing the new meal
#[post("/meals")]
pub async fn create_meal(req: HttpRequest, mut payload: Payload) -> impl Responder {

    /// Check if the Content-Type is application/json
    ///
    /// If it is not, return a [HttpResponse::UnsupportedMediaType] with a Error Code 0
    match req.headers().get("Content-Type") {
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

    /// Read the body of the request
    ///
    /// If the body is too large, return a [HttpResponse::PayloadTooLarge] with a Error Code -6
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

    /// Deserialize the body into a [NewMeal] struct
    let body = from_slice::<NewMeal>(&body);

    /// Check if the deserialization was successful
    let body = match body {
        Ok(body) => body,
        Err(e) => {
            /// If it was not, return a [HttpResponse::UnsupportedMediaType] with a Error Code -1
            return HttpResponse::BadRequest().json(json!({
                "message": "One or more required fields are missing or invalid",
                "id": "-1"
            }))
        }
    };

    /// Create a connection to the database
    let conn = &mut establish_connection();

    ///Check if the meal with the same name already exists
    ///
    /// If it does, return a [HttpResponse::UnprocessableEntity] with a Error Code -2
    let meal_exists = meals.filter(name.eq(&*body.name)).select(name).first::<String>(conn);
    match meal_exists {
        Ok(meal_exists) => {
            return HttpResponse::UnprocessableEntity().json(json!({
                "message": "Meal with the same name already exists",
                "id": "-2"
            }))
        }
        Err(e) => {
            // Continue
        }
    };

    /// Insert [NewMeal] into the database
    let new_meal = insert_into(meals).values(&body).get_result::<Meal>(conn);
    /// Check if the insertion was successful
    ///
    /// If it was not, return a [HttpResponse::UnprocessableEntity] with a Error Code -6
    let new_meal = match new_meal {
        Ok(new_meal) => new_meal,
        Err(e) => {
            return HttpResponse::UnprocessableEntity().json(json!({
                "message": "One or more required fields are missing or invalid",
                "id": "-6"
            }))
        }
    };

    /// Get the ID of the newly inserted meal
    ///
    /// If retrieving the ID fails, return a [HttpResponse::InternalServerError] with a Error Code -5
    ///
    /// TODO: Find a better way to get the ID of the newly inserted dish
    let new_meal_id = meals.filter(name.eq(&*new_meal.name)).select(meal_id).first::<i32>(conn);
    let new_meal_id = match new_meal_id {
        Ok(new_meal_id) => new_meal_id,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message": "Error getting meal ID",
                "id": "-5"
            }))
        }
    };

    /// Return a [HttpResponse::Created] with a JSON body containing the ID of the new dish
    HttpResponse::Created().json(json!({
        "message": "Meal created successfully",
        "id": new_meal_id,
    }))

}