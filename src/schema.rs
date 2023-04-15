// @generated automatically by Diesel CLI.

diesel::table! {
    dish (dish_id) {
        dish_id -> Int4,
        name -> Varchar,
        calories -> Float8,
        sodium -> Float8,
        sugar -> Float8,
        serving_size -> Float8,
    }
}
