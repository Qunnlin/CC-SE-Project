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

/// Struct to represent a new diet to be inserted into the database or requested by the user
#[derive( Insertable, Serialize, Deserialize)]
#[diesel(table_name = diets)]
pub struct NewDiet {
    pub name: String,
    pub cal: f64,
    pub sodium: f64,
    pub sugar: f64,
}
