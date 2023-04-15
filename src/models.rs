use crate::schema::dishes;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Dish {
    pub dish_id: i32,
    pub name: String,
    pub calories: f64,
    pub sodium: f64,
    pub sugar: f64,
    pub serving_size: f64,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = dishes)]
pub struct NewDish {
    pub name: String,
    pub calories: f64,
    pub sodium: f64,
    pub sugar: f64,
    pub serving_size: f64,

}

