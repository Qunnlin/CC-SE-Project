#![allow(unused_doc_comments)]

/// Actix Imports
use actix_web::{get, post, delete, put, HttpResponse, Responder, HttpRequest, web};
use actix_web::web::{Data, Query};

/// Diesel Imports
use diesel;
use diesel::prelude::*;
use diesel::{delete, insert_into, QueryDsl, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};

/// Misc Imports
use std::string::ToString;
use serde_json::{from_str, json};

/// Module Imports
use super::models::{Meal, NewMeal, Diet};
use super::diet_client::get_diet_by_name;

/// Crate Imports
use crate::schema::meals::dsl::*;
use crate::db::DbPool;
use crate::meals::models::DietQuery;

/// Error codes as defined in the Assigment
const NOT_JSON: &str = "0";
const PARAM_NOT_FOUND: &str = "-1";
const MEAL_ALREADY_EXISTS: &str = "-2";
const MEAL_NOT_FOUND: &str = "-5";
const DISH_ID_NOT_FOUND: &str = "-6";
const DIET_NOT_FOUND: &str = "-7";
const INTERNAL_SERVER_ERROR: &str = "-8";

/// Disallow DELETE requests to the /meals route
/// Returns a [HttpResponse::MethodNotAllowed] with a JSON body containing an error message and the error code -7
#[delete("/meals")]
pub async fn meals_collection_deletion() -> impl Responder {
    /// Return a [HttpResponse::MethodNotAllowed] with a JSON body containing an error message and the error code -7
    HttpResponse::MethodNotAllowed().json(json!({
        "message": "Method not allowed",
    }))
}

/*
=============================== GET /meals ===============================
 */
/// # Creates the route for getting all meals in "/meals"
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `query` - A optional [web::Query<String>] containing the diet to filter the meals by
/// ## Returns
/// * [HttpResponse] with a status of 200 and a JSON body containing all meals
#[get("/meals")]
pub async fn get_all_meals(db_pool: Data<DbPool>, query: Query<DietQuery>) -> impl Responder {

    /// Check if the diet query parameter is present and is not empty
    if let Some(diet_name) = &query.diet {
        if diet_name.is_empty() {
            /// Establish a connection to the database
            let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();
            /// Get all meals from the database
            let results = meals.load::<Meal>(conn).expect("Error loading meals");
            /// Return a 200 response with the meals in the body
            HttpResponse::Ok().json(results)
        } else {

            /// Send GET request to "/diet/{diet}"
            let diet = get_diet_by_name(diet_name).await;
            /// If the diet is found parse the response body into a Diet struct
            /// If the diet is not found, return a 404 response with a JSON body containing an error message and the error code -7
            let diet: Diet = match diet {
                Ok(diet) => {
                    match from_str(&diet) {
                        Ok(diet) => diet,
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(json!({
                            "message": "Internal Server Error",
                            "error_code": INTERNAL_SERVER_ERROR,
                            "error": e.to_string()
                        }))
                        }
                    }
                },
                Err(e) => {
                    return HttpResponse::NotFound().json(json!({
                    "message": "Diet not found",
                    "error_code": DIET_NOT_FOUND,
                    "error": e.to_string()
                }))
                }
            };

            /// Establish a connection to the database
            let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();
            /// Get all meals from the database that have <= the calories, sodium, and sugar of the diet
            let results = meals
                .filter(cal.le(&diet.cal))
                .filter(sodium.le(&diet.sodium))
                .filter(sugar.le(&diet.sugar))
                .load::<Meal>(conn);
            /// If there is an error loading the meals, return a 500 response with a JSON body containing an error message and the error code -8
            return match results {
                Ok(results) => HttpResponse::Ok().json(results),
                Err(e) => {
                    HttpResponse::InternalServerError().json(json!({
                    "message": "Internal Server Error",
                    "error_code": INTERNAL_SERVER_ERROR,
                    "error": e.to_string()
                }))
                }
            };
        }
    } else {
        /// Establish a connection to the database
        let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();
        /// Get all meals from the database
        let results = meals.load::<Meal>(conn).expect("Error loading meals");
        /// Return a 200 response with the meals in the body
        HttpResponse::Ok().json(results)
    }

}

/*
=============================== POST /meals ===============================
 */
/// # Creates the route for creating a meal in "/meals"
/// Creates a new meal in the database, based on the JSON body of the request
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `req` - A [HttpRequest] containing the request
/// * `new_meal` - A [web::Json<NewMeal>] containing the JSON body of the request
/// ## Returns
/// * [HttpResponse] with a status of 201 and a JSON body containing the new meal
#[post("/meals")]
pub async fn create_meal(db_pool: web::Data<DbPool>, req: HttpRequest, new_meal: web::Json<NewMeal>) -> impl Responder {

    /// Check if the Content-Type is application/json
    ///
    /// If it is not, return a [HttpResponse::UnsupportedMediaType] with a Error Code 0
    match req.headers().get("Content-Type") {
        Some(content_type) => {
            if content_type != "application/json" {
                return HttpResponse::UnsupportedMediaType().body(NOT_JSON)
            }
        },
        None => {
            return HttpResponse::UnsupportedMediaType().body(NOT_JSON)
        }
    }

    /// Check if new_meal is has all the required fields
    /// If it does not, return a [HttpResponse::UnprocessableEntity] with a Error Code -1
    if new_meal.name.is_empty() || new_meal.appetizer.is_none() || new_meal.main.is_none() || new_meal.dessert.is_none() {
        return HttpResponse::UnprocessableEntity().body(PARAM_NOT_FOUND)
    }

    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();

    ///Check if the meal with the same name already exists
    /// If it does, return a [HttpResponse::UnprocessableEntity] with a Error Code -2
    let meal_exists = meals.filter(name.eq(&*new_meal.name)).select(name).first::<String>(conn);
    match meal_exists {
        Ok(e) => {
            eprintln!("Meal already exists: {}", e);
            return HttpResponse::UnprocessableEntity().body(MEAL_ALREADY_EXISTS)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            // Continue
        }
    };

    /// Insert [NewMeal] into the database
    let new_meal = insert_into(meals).values(&*new_meal).get_result::<Meal>(conn);
    /// Check if the insertion was successful
    ///
    /// If it was not, return a [HttpResponse::UnprocessableEntity] with a Error Code -6
    let new_meal = match new_meal {
        Ok(new_meal) => new_meal,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::UnprocessableEntity().body(DISH_ID_NOT_FOUND)
        }
    };

    /// Get the ID of the newly inserted meal
    ///
    /// If retrieving the ID fails, return a [HttpResponse::InternalServerError] with a Error Code -5
    ///
    /// TODO: Find a better way to get the ID of the newly inserted dish
    let new_meal_id = meals.filter(name.eq(&*new_meal.name)).select(id).first::<i32>(conn);
    let new_meal_id = match new_meal_id {
        Ok(new_meal_id) => new_meal_id,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().body(MEAL_NOT_FOUND)
        }
    };

    /// Return a [HttpResponse::Created] with a JSON body containing the ID of the new dish
    HttpResponse::Created().body(new_meal_id.to_string())

}

/*
=============================== GET /meals/{id} ===============================
 */

/// # Creates the route for getting a meal by ID in "/meals/{id}"
/// # Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `req_id` - A [web::Path<i32>] containing the ID of the meal
/// # Returns
/// * [HttpResponse::Ok] with a JSON body containing the meal
#[get("/meals/{id:\\d+}")]
pub async fn get_meal(db_pool: Data<DbPool>, req_id: web::Path<i32>) -> impl Responder {

    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();

    /// Get the meal with the specified ID
    let meal = meals.find(&*req_id).first::<Meal>(conn);

    /// Check if the meal exists
    ///
    /// If it does not, return a [HttpResponse::NotFound] with a Error Code -3
    let meal = match meal {
        Ok(meal) => meal,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(MEAL_NOT_FOUND)
        }
    };

    /// Return a [HttpResponse::Ok] with a JSON body containing the meal
    HttpResponse::Ok().json(meal)
}

/*
=============================== GET /meals/{name} ===============================
 */
/// # Creates the route for getting a meal by name in "/meals/{name}"
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `meal_name` - A [web::Path<String>] containing the name of the meal
/// ## Returns
/// * [HttpResponse::Ok] with a JSON body containing the meal
#[get("/meals/{name:.*}")]
pub async fn get_meal_by_name(db_pool: Data<DbPool>, meal_name: web::Path<String>) -> impl Responder {

    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();

    /// Get the meal with the specified name
    let meal = meals.filter(name.eq(&*meal_name)).first::<Meal>(conn);

    /// Check if the meal exists
    ///
    /// If it does not, return a [HttpResponse::NotFound] with a Error Code -3
    let meal = match meal {
        Ok(meal) => meal,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(MEAL_NOT_FOUND)
        }
    };

    /// Return a [HttpResponse::Ok] with a JSON body containing the meal
    HttpResponse::Ok().json(meal)
}

/*
=============================== DELETE /meals/{id} ===============================
 */
/// # Creates the route for deleting a meal by ID in "/meals/{id}"
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `req_id` - A [web::Path<i32>] containing the ID of the meal
/// ## Returns
/// * [HttpResponse::Ok] with a JSON body containing the ID of the deleted meal
#[delete("/meals/{id:\\d+}")]
pub async fn delete_meal(db_pool: Data<DbPool>, req_id: web::Path<i32>) -> impl Responder {

    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();

    /// Check if the meal exists
    /// If it does not, return a [HttpResponse::NotFound] with a Error Code -5
    let meal_exists = meals.find(&*req_id).select(id).first::<i32>(conn);
    match meal_exists {
        Ok(meal_exists) => meal_exists,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(MEAL_NOT_FOUND)
        }
    };

    /// Delete the meal with the specified ID
    let meal = delete(meals.find(&*req_id)).execute(conn);

    /// Check if the meal exists
    ///
    /// If it does not, return a [HttpResponse::NotFound] with a Error Code -3
    match meal {
        Ok(meal) => meal,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message": "Error deleting meal",
            }))
        }
    };

    /// Return a [HttpResponse::Ok] with a JSON body containing the ID of the deleted meal
    HttpResponse::Ok().body(req_id.to_string())
}

/*
=============================== DELETE /meals/{name} ===============================
 */

/// # Creates the route for deleting a meal by name in "/meals/{name}"
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `meal_name` - A [web::Path<String>] containing the name of the meal
/// ## Returns
/// * [HttpResponse::Ok] with a JSON body containing the ID of the deleted meal

#[delete("/meals/{name:.*}")]
pub async fn delete_meal_by_name(db_pool: Data<DbPool>, meal_name: web::Path<String>) -> impl Responder {

    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();

    /// Check if the meal exists
    /// If it does, save the ID of the meal,
    /// If it does not, return a [HttpResponse::NotFound] with a Error Code -5
    let meal_exists = meals.filter(name.eq(&*meal_name)).select(id).first::<i32>(conn);
    let deleted_id = match meal_exists {
        Ok(meal_exists) => meal_exists,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(MEAL_NOT_FOUND)
        }
    };

    /// Delete the meal with the specified name
    let meal = delete(meals.filter(name.eq(&*meal_name))).execute(conn);

    /// Check if deletion was successful
    /// If it was not, return a [HttpResponse::InternalServerError] with a Error Code -8
    match meal {
        Ok(meal) => meal,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message": "Error deleting meal",
            }))
        }
    };

    /// Return a [HttpResponse::Ok] with a JSON body containing the ID of the deleted meal
    HttpResponse::Ok().body(deleted_id.to_string())
}

/*
=============================== PUT /meals/{id} ===============================
 */
/// # Creates the route for updating a meal by ID in "/meals/{id}"
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `req` - The [HttpRequest] object
/// * `req_id` - A [web::Path<i32>] containing the ID of the meal
/// * `new_meal` - A [web::Json<NewMeal>] containing the new meal data
/// ## Returns
/// * [HttpResponse::Ok] on success
/// * [HttpResponse::UnsupportedMediaType] if the Content-Type is not application/json
/// * [HttpResponse::UnprocessableEntity] if the new meal data is missing required fields
/// * [HttpResponse::InternalSexexrrverError] on failure
/// * [HttpResponse::NotFound] if the meal does not exist

#[put("/meals/{id:\\d+}")]
pub async fn update_meal(db_pool: Data<DbPool>, req: HttpRequest, req_id: web::Path<i32>, new_meal: web::Json<NewMeal>) -> impl Responder {

    /// Check if the Content-Type is application/json
    ///
    /// If it is not, return a [HttpResponse::UnsupportedMediaType] with a Error Code 0
    match req.headers().get("Content-Type") {
        Some(content_type) => {
            if content_type != "application/json" {
                return HttpResponse::UnsupportedMediaType().body(NOT_JSON)
            }
        },
        None => {
            return HttpResponse::UnsupportedMediaType().body(NOT_JSON)
        }
    }

    /// Check if new_meal is has all the required fields
    /// If it does not, return a [HttpResponse::UnprocessableEntity] with a Error Code -1
    if new_meal.name.is_empty() || new_meal.appetizer.is_none() || new_meal.main.is_none() || new_meal.dessert.is_none() {
        return HttpResponse::UnprocessableEntity().body(PARAM_NOT_FOUND)
    }

    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();

    /// Check if the meal exists
    /// If it does not, return a [HttpResponse::NotFound] with a Error Code -5
    let meal_exists = meals.find(&*req_id).select(id).first::<i32>(conn);
    match meal_exists {
        Ok(meal_exists) => meal_exists,
        Err(e) => {

            // Can use this to create a new meal, but not required in assignment

            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(MEAL_NOT_FOUND)
        }
    };

    /// Update the meal with the specified ID
    let meal = diesel::update(meals.find(&*req_id))
        .set((
            name.eq(&new_meal.name),
            appetizer.eq(&new_meal.appetizer),
            main.eq(&new_meal.main),
            dessert.eq(&new_meal.dessert),

        ))
        .execute(conn);

    /// Check if the meal exists
    ///
    /// If it does not, return a [HttpResponse::NotFound] with a Error Code -3
    match meal {
        Ok(meal) => meal,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "message": "Error updating meal",
            }))
        }
    };

    /// Return a [HttpResponse::Ok] with a JSON body containing the ID of the updated meal
    HttpResponse::Ok().body(req_id.to_string())
}