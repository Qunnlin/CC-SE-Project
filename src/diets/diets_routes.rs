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

/// Module imports
use super::models::{Diet, NewDiet};

/// Crate imports
use crate::db::DbPool;
use crate::schema::diets::dsl::*;


/// Disallow DELETE requests to the /diets route
/// Returns a [HttpResponse::MethodNotAllowed] with a JSON body containing an error message and the error code -7
#[delete("/diets")]
pub async fn diets_collection_deletion() -> impl Responder {
    /// Return a [HttpResponse::MethodNotAllowed] with a JSON body containing an error message and the error code -7
    HttpResponse::MethodNotAllowed().body("Method not allowed")
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
    let results = diets.load::<Diet>(conn);

    return match results {
        Ok(results) => {

            /// Convert Vec<Diet> to Vec<NewDiet>
            let results: Vec<NewDiet> = results.into_iter().map(|diet| NewDiet {
                name: diet.name,
                cal: diet.cal,
                sodium: diet.sodium,
                sugar: diet.sugar,
            }).collect();

            /// Return a 200 response with the diets in the body
            HttpResponse::Ok().json(results)
        },
        Err(e) => {
            /// Return a 500 response with a JSON body containing an error message and the error code -6
            HttpResponse::InternalServerError().body(format!("Error querying the database: {}", e))
        }
    }
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
    /// If it is not, return a [HttpResponse::UnsupportedMediaType]
    match req.headers().get("Content-Type") {
        Some(content_type) => {
            if content_type != "application/json" {
                return HttpResponse::UnsupportedMediaType().body("POST expects content type to be application/json")
            }
        },
        None => {
            return HttpResponse::UnsupportedMediaType().body("POST expects content type to be application/json")
        }
    }

    /// Check if new_diet is has all the required fields
    /// If it does not, return a [HttpResponse::UnprocessableEntity] with a Error Code -1
    if new_diet.name.is_empty() || new_diet.cal.is_nan() || new_diet.sodium.is_nan() || new_diet.sugar.is_nan() {
        return HttpResponse::UnprocessableEntity().body("Incorrect POST format")
    }

    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();

    ///Check if the diet with the same name already exists
    /// If it does, return a [HttpResponse::UnprocessableEntity] with a Error Code -2
    let diet_exists = diets.filter(name.eq(&*new_diet.name)).select(name).first::<String>(conn);
    match diet_exists {
        Ok(e) => {
            return HttpResponse::UnprocessableEntity().body("Diet with name {} already exists".replace("{}", &*e))
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
            return HttpResponse::InternalServerError().body("Failed to insert new diet")
        }
    };

    /// Return a [HttpResponse::Created] with a JSON body containing the ID of the new diet
    HttpResponse::Created().body("Diet {} was created successfully".replace("{}", &*new_diet.name))

}

/*
=============================== GET /diets/{id} ===============================
 */
/// # Creates the route for getting a diet by ID in "/diets/{id}"
/// Gets a diet from the database, based on the ID in the URL
/// ## Arguments
/// * `db_pool` - A [web::Data<DbPool>] containing the connection pool to the database
/// * `req_id` - A [web::Path<i32>] containing the ID of the diet
/// ## Returns
/// * [HttpResponse] with a status of 200 and a JSON body containing the diet
#[get("/diets/{id:\\d+}")]
pub async fn get_diet_by_id(db_pool: web::Data<DbPool>, req_id: web::Path<i32>) -> impl Responder {
    /// Create a connection to the database
    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut db_pool.get().unwrap();
    /// Get the diet from the database
    let diet = diets.find(&*req_id).first::<Diet>(conn);
    /// Check if the diet was found
    ///
    /// If it was not, return a [HttpResponse::NotFound] with a Error Code -5
    return match diet {
        Ok(diet) => {
            /// Only return name, cal, sodium, and sugar
            let diet = NewDiet {
                name: diet.name,
                cal: diet.cal,
                sodium: diet.sodium,
                sugar: diet.sugar,
            };
            HttpResponse::Ok().json(diet)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::NotFound().body("Diet {} not found".replace("{}", &*req_id.to_string()))
        }
    };
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
    /// If it was, return a [HttpResponse::Ok] with a JSON body containing the diet
    /// If it was not, return a [HttpResponse::NotFound] with a Error Code -5
    return match result {
        Ok(result) => {
            /// Only return name, cal, sodium, and sugar
            let diet = NewDiet {
                name: result.name,
                cal: result.cal,
                sodium: result.sodium,
                sugar: result.sugar,
            };
            HttpResponse::Ok().json(diet)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::NotFound().body("Diet {} not found".replace("{}", &*diet_name))
        }
    };

}
