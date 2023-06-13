#![allow(unused_doc_comments)]

use crate::schema::diets;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

/// Diet struct to represent a diet in the database
#[derive(Queryable, Serialize, Deserialize)]
pub struct Diet {
    pub id: i32,
    pub name: String,
    pub cal: f64,
    pub sodium: f64,
    pub sugar: f64,
}

/// Struct to represent a diet reqested by the user, either for creation or deletion
/// The Values are optionals since a user might not include all of them and we need to be able to handle that
/// Translates to a NewDiet struct if all values are present
#[derive(Deserialize)]
pub struct ReqDiet {
    pub name: Option<String>,
    pub cal: Option<f64>,
    pub sodium: Option<f64>,
    pub sugar: Option<f64>,
}

/// Struct that represents a new diet to be inserted into the database or served to the user
/// Translates to a Diet struct on insertion
#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = diets)]
pub struct NewDiet {
    pub name: String,
    pub cal: f64,
    pub sodium: f64,
    pub sugar: f64,
}
