#![allow(unused_doc_comments)]

/// Actix imports
use actix_web::{get, post, delete, HttpResponse, Responder, HttpRequest, web};
use actix_web::web::{Data};

/// Diesel imports
use diesel;
use diesel::prelude::*;
use diesel::{insert_into, QueryDsl, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

/// Misc imports
use serde_json::{json};

/// Module imports
use super::models::{Diet, NewDiet};

/// Crate imports
use crate::db::DbPool;
use crate::schema::diets::dsl::*;


/// Error codes as defined in the Assigment
const NOT_JSON: &str = "0";
const PARAM_NOT_FOUND: &str = "-1";
const DIET_ALREADY_EXISTS: &str = "-2";
const DIET_NOT_FOUND: &str = "-5";
const INTERNAL_SERVER_ERROR: &str = "-6";

/// Disallow DELETE requests to the /diets route
/// Returns a [HttpResponse::MethodNotAllowed] with a JSON body containing an error message and the error code -7
#[delete("/diets")]
pub async fn diets_collection_deletion() -> impl Responder {
    /// Return a [HttpResponse::MethodNotAllowed] with a JSON body containing an error message and the error code -7
    HttpResponse::MethodNotAllowed().json(json!({
        "message": "Method not allowed",
    }))
}

/*
=============================== GET /diets ===============================
 */
/// # Creates the route for getting all diets in "/diets"
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// ## Returns
/// * [HttpResponse] with a status of 200 and a JSON body containing all diets
#[get("/diets")]
pub async fn get_all_diets(db_pool: Data<DbPool>) -> impl Responder {
    /// Establish a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();
    /// Get all meals from the database
    let results = diets.load::<Diet>(conn).expect("Error loading meals");
    /// Return a 200 response with the meals in the body
    HttpResponse::Ok().json(results)
}

/*
=============================== POST /diets ===============================
 */
/// # Creates the route for creating a diet in "/diets"
/// Creates a new diet in the database, based on the JSON body of the request
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `req` - A [HttpRequest] containing the request
/// * `new_diet` - A [web::Json<NewDiet>] containing the JSON body of the request
/// ## Returns
/// * [HttpResponse] with a status of 201 and a JSON body containing the new meal
#[post("/diets")]
pub async fn create_diet(db_pool: web::Data<DbPool>, req: HttpRequest, new_diet: web::Json<NewDiet>) -> impl Responder {

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

    /// Check if new_diet is has all the required fields
    /// If it does not, return a [HttpResponse::UnprocessableEntity] with a Error Code -1
    if new_diet.name.is_empty() || new_diet.cal.is_nan() || new_diet.sodium.is_nan() || new_diet.sugar.is_nan() {
        return HttpResponse::UnprocessableEntity().body(PARAM_NOT_FOUND)
    }

    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();

    ///Check if the diet with the same name already exists
    /// If it does, return a [HttpResponse::UnprocessableEntity] with a Error Code -2
    let diet_exists = diets.filter(name.eq(&*new_diet.name)).select(name).first::<String>(conn);
    match diet_exists {
        Ok(e) => {
            eprintln!("Diet already exists: {}", e);
            return HttpResponse::UnprocessableEntity().body(DIET_ALREADY_EXISTS)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            // Continue
        }
    };

    /// Insert [NewDiet] into the database
    let new_diet = insert_into(diets).values(&*new_diet).get_result::<Diet>(conn);
    /// Check if the insertion was successful
    ///
    /// If it was not, return a [HttpResponse::UnprocessableEntity] with a Error Code -6
    let new_diet = match new_diet {
        Ok(new_diet) => new_diet,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::UnprocessableEntity().body(INTERNAL_SERVER_ERROR)
        }
    };

    /// Get the ID of the newly inserted diet
    ///
    /// If retrieving the ID fails, return a [HttpResponse::InternalServerError] with a Error Code -5
    ///
    /// TODO: Find a better way to get the ID of the newly inserted diet
    let new_diet_id = diets.filter(name.eq(&*new_diet.name)).select(diet_id).first::<i32>(conn);
    let new_diet_id = match new_diet_id {
        Ok(new_diet_id) => new_diet_id,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::InternalServerError().body(DIET_NOT_FOUND)
        }
    };

    /// Return a [HttpResponse::Created] with a JSON body containing the ID of the new diet
    HttpResponse::Created().body(new_diet_id.to_string())

}

/*
=============================== GET /diets/{id} ===============================
 */
/// # Creates the route for getting a diet by ID in "/diets/{id}"
/// Gets a diet from the database, based on the ID in the URL
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `id` - A [web::Path<i32>] containing the ID of the diet
/// ## Returns
/// * [HttpResponse] with a status of 200 and a JSON body containing the diet
#[get("/diets/{id:\\d+}")]
pub async fn get_diet_by_id(db_pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();
    /// Get the diet from the database
    let diet = diets.find(&*id).first::<Diet>(conn);
    /// Check if the diet was found
    ///
    /// If it was not, return a [HttpResponse::NotFound] with a Error Code -5
    let result = match diet {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(DIET_NOT_FOUND)
        }
    };
    /// Return a [HttpResponse::Ok] with a JSON body containing the diet
    HttpResponse::Ok().json(result)
}

/*
=============================== GET /diets/{name} ===============================
 */
/// # Creates the route for getting a diet by name in "/diets/{name}"
/// Gets a diet from the database, based on the name in the URL
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `name` - A [web::Path<String>] containing the name of the diet
/// ## Returns
/// * [HttpResponse] with a status of 200 and a JSON body containing the diet
#[get("/diets/{name:.*}")]
pub async fn get_diet_by_name(db_pool: web::Data<DbPool>, diet_name: web::Path<String>) -> impl Responder {
    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();
    /// Get the diet from the database
    let result = diets.filter(name.eq(&*diet_name)).first::<Diet>(conn);
    /// Check if the diet was found
    ///
    /// If it was not, return a [HttpResponse::NotFound] with a Error Code -5
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {}", e);
            return HttpResponse::NotFound().body(DIET_NOT_FOUND)
        }
    };
    /// Return a [HttpResponse::Ok] with a JSON body containing the diet
    HttpResponse::Ok().json(result)
}
