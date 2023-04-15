use super::db::DbConn;
use rocket_contrib::json::Json;
use super::models::{Dish, NewDish};
use serde_json::{json, Value};
use super::ninjas_api::get_nutrition_info;

use rocket::get;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}


