// @generated automatically by Diesel CLI.

diesel::table! {
    dishes (dish_id) {
        dish_id -> Int4,
        name -> Varchar,
        calories -> Float8,
        sodium -> Float8,
        sugar -> Float8,
        serving_size -> Float8,
    }
}

diesel::table! {
    meals (meal_id) {
        meal_id -> Int4,
        name -> Varchar,
        appetizer -> Nullable<Int4>,
        entree -> Nullable<Int4>,
        dessert -> Nullable<Int4>,
        cal -> Nullable<Float8>,
        sodium -> Nullable<Float8>,
        sugar -> Nullable<Float8>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    dishes,
    meals,
);
