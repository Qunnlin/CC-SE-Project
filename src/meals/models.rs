#![allow(unused_doc_comments)]

use crate::schema::dishes;
use crate::schema::meals;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

/// Dish struct to represent a dish in the database
/// ID should be snake case but the assignment wants UPPER CASE, sooooo...
#[derive(Queryable, Serialize, Deserialize)]
pub struct Dish {
    pub ID: i32,
    pub name: String,
    pub cal: f64,
    pub sodium: f64,
    pub sugar: f64,
    pub size: f64,
}

/// Struct to represent a new dish to be inserted into the database
/// Translates to a Dish struct on insertion
#[derive(Insertable, Deserialize)]
#[diesel(table_name = dishes)]
pub struct NewDish {
    pub name: String,
    pub cal: f64,
    pub sodium: f64,
    pub sugar: f64,
    pub size: f64,

}

/// Struct that represents a dish reqested by the user, either for creation or deletion
/// The Values are optionals since a user might not include all of them and we need to be able to handle that
/// Translates to a NewDish struct if all values are present
#[derive(Deserialize)]
pub struct ReqDish {
    pub name: Option<String>,
}


/// Meal struct to represent a meal in the database
/// ID should be snake case but the assignment wants UPPER CASE, sooooo...
#[derive(Queryable, Serialize, Deserialize)]
pub struct Meal {
    pub ID: i32,
    pub name: String,
    pub appetizer: Option<i32>,
    pub main: Option<i32>,
    pub dessert: Option<i32>,
    pub cal: Option<f64>,
    pub sodium: Option<f64>,
    pub sugar: Option<f64>,
}

/// Struct to represent a new meal to requested by the user
/// The Values are optionals since a user might not include all of them and we need to be able to handle that
/// Translates to a NewMeal struct if all values are present
#[derive(Deserialize)]
pub struct ReqMeal {
    pub name: Option<String>,
    pub appetizer: Option<i32>,
    pub main: Option<i32>,
    pub dessert: Option<i32>,
}

/// Struct to represent a new meal to be inserted into the database
/// Translates to a Meal struct on insertion
#[derive(Insertable, Deserialize)]
#[diesel(table_name = meals)]
pub struct NewMeal {
    pub name: String,
    pub appetizer: i32,
    pub main: i32,
    pub dessert: i32,
}

/// Struct that represents a meal reqested by the user
#[derive(Deserialize)]
pub struct ReqDiet {
    pub diet: Option<String>,
}
/// Struct that represents a diet
#[derive(Deserialize)]
pub struct Diet {
    pub name: String,
    pub cal: f64,
    pub sodium: f64,
    pub sugar: f64,
}
