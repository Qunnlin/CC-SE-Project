use crate::schema::dishes;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

/// Dish struct to represent a dish in the database
#[derive(Queryable, Serialize, Deserialize)]
pub struct Dish {
    pub dish_id: i32,
    pub name: String,
    pub calories: f64,
    pub sodium: f64,
    pub sugar: f64,
    pub serving_size: f64,
}

/// Struct to represent a new dish to be inserted into the database
#[derive(Insertable, Deserialize)]
#[diesel(table_name = dishes)]
pub struct NewDish {
    pub name: String,
    pub calories: f64,
    pub sodium: f64,
    pub sugar: f64,
    pub serving_size: f64,

}

/// Struct that represents a dish reqested by the user, either for creation or deletion
///
/// The serving size is optional, and defaults to 100g
#[derive(Deserialize)]
pub struct ReqDish {
    pub name: String,
    #[serde(default = "default_serving_size")]
    pub serving_size: f64
}


/// Function to return the default serving size => 100g
fn default_serving_size() -> f64 {
    100.0
}