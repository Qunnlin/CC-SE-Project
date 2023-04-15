use diesel;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use super::schema::dish::dsl::dish as all_dishes;
use super::schema::dish;

// Our Dish struct, which represents a row in the dish table
#[derive(Serialize, Queryable)]
pub struct Dish {
    pub dish_id: i32,
    pub name: String,
    pub calories: f32,
    pub sodium: f32,
    pub sugar: f32,
    pub serving_size: f32,
}

// This struct is used to retrieve a dish from the database
#[derive(Deserialize)]
pub struct DishData {
    pub name: String,
}

// This struct is used to insert a dish into the database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "dish"]
pub struct NewDish {
    pub name: String,
    pub calories: f32,
    pub sodium: f32,
    pub sugar: f32,
    pub serving_size: f32,
}


// This implementation contains all the methods that interact with the database
impl Dish {

    pub fn get_all_dishes(conn: Conn) -> Vec<Dish> {
        all_dishes
            .order(dish::dish_id.desc())
            .load::<Dish>(conn)
            .expect("Error loading dishes")
    }


    pub fn get_dish_by_id(conn: Conn, dish_id: i32) -> Dish {
        all_dishes
            .find(dish_id)
            .first::<Dish>(conn)
            .expect("Error loading dish")
    }

    pub fn insert_dish(conn: Conn, new_dish: NewDish) -> i32 {
        insert_into(dish::table)
            .values(&new_dish)
            .execute(conn)
            .expect("Error inserting dish")
    }

}


