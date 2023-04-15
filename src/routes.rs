use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{Dish, NewDish};
use serde_json::Value;
use crate::models::DishData;

use crate::ninjas_api::get_nutrition_info;

// GET /dishes returns all dishes indexed by dish_id
#[get("/dishes")]
pub fn get_all_dishes(conn: Box<DbConn>) -> Json<Value> {
    let dishes = Dish::get_all_dishes(&conn);
    Json(json!({
        "status": "success",
        "dishes": dishes
    }))
}

// POST /dishes adds a dish of teh given name. On success, returns the dish_id and code 201
#[post("/dishes", data = "<dish_data>")]
pub async fn add_dish(conn: Box<DbConn>, dish_data: Json<DishData>) -> Json<Value> {
    let dish_name = dish_data.name.clone();
    let nutrition_info = get_nutrition_info(&dish_name).await.get(0).unwrap();
    let dish = NewDish {
        name: dish_name,
        calories: nutrition_info.calories as f32,
        sodium: nutrition_info.sodium_mg as f32,
        sugar: nutrition_info.sugar_g as f32,
        serving_size: nutrition_info.serving_size_g as f32,
    };
    let dish_id = Dish::insert_dish(&conn, dish);
    Json(json!({
        "status": "success",
        "dish_id": dish_id
    }))
}

// GET /dishes/<dish_id> returns the dish with the given dish_id
#[get("/dishes/<dish_id>")]
pub fn get_dish(conn: Box<DbConn>, dish_id: i32) -> Json<Value> {
    let dish = Dish::get_dish(&conn, dish_id);
    Json(json!({
    "status": "success",
    "dish": dish
}))
}

// GET /dishes/<dish_name> returns the dish with the given dish_name
#[get("/dishes/<dish_name>")]
pub async fn get_dish_by_name(conn: Box<DbConn>, dish_name: String) -> Json<Value> {
    let dish = Dish::get_dish_by_name(&conn, dish_name);
    Json(json!({
    "status": "success",
    "dish": dish
}))
}

// PUT /dishes/<dish_id> updates the dish with the given dish_id
#[put("/dishes/<dish_id>", data = "<dish_data>")]
pub async fn update_dish(conn: Box<DbConn>, dish_id: i32, dish_data: Json<DishData>) -> Json<Value> {
    let dish_name = dish_data.name.clone();
    let nutrition_info = get_nutrition_info(&dish_name).await.get(0).unwrap();
    let dish = NewDish {
        name: dish_name,
        calories: nutrition_info.calories as f32,
        sodium: nutrition_info.sodium_mg as f32,
        sugar: nutrition_info.sugar_g as f32,
        serving_size: nutrition_info.serving_size_g as f32,
    };
    Dish::update_dish(&conn, dish_id, dish);
    Json(json!({
        "status": "success",
        "dish_id": dish_id
    }))
}

// PUT /dishes/<dish_name> updates the dish with the given dish_name
#[put("/dishes/<dish_name>", data = "<dish_data>")]
pub async fn update_dish_by_name(conn: Box<DbConn>, dish_name: String, dish_data: Json<DishData>) -> Json<Value> {
    let nutrition_info = get_nutrition_info(&dish_name).await.get(0).unwrap();
    let dish = NewDish {
        name: dish_name,
        calories: nutrition_info.calories as f32,
        sodium: nutrition_info.sodium_mg as f32,
        sugar: nutrition_info.sugar_g as f32,
        serving_size: nutrition_info.serving_size_g as f32,
    };
    let dish_name = dish_data.name.clone();
    Dish::update_dish_by_name(&conn, dish_name, dish);
    Json(json!({
        "status": "success",
        "dish_name": dish_name
    }))
}

// DELETE /dishes/<dish_id> deletes the dish with the given dish_id
#[delete("/dishes/<dish_id>")]
pub fn delete_dish(conn: Box<DbConn>, dish_id: i32) -> Json<Value> {
    Dish::delete_dish(&conn, dish_id);
    Json(json!({
        "status": "success",
        "dish_id": dish_id
    }))
}

// DELETE /dishes/<dish_name> deletes the dish with the given dish_name
#[delete("/dishes/<dish_name>")]
pub fn delete_dish_by_name(conn: Box<DbConn>, dish_name: String) -> Json<Value> {
    Dish::delete_dish_by_name(&conn, dish_name);
    Json(json!({
        "status": "success",
        "dish_name": dish_name
    }))
}



