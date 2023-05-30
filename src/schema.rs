// @generated automatically by Diesel CLI.

diesel::table! {
    diets (id) {
        id -> Int4,
        name -> Varchar,
        cal -> Float8,
        sodium -> Float8,
        sugar -> Float8,
    }
}

diesel::table! {
    dishes (id) {
        id -> Int4,
        name -> Varchar,
        cal -> Float8,
        sodium -> Float8,
        sugar -> Float8,
        size -> Float8,
    }
}

diesel::table! {
    meals (id) {
        id -> Int4,
        name -> Varchar,
        appetizer -> Nullable<Int4>,
        main -> Nullable<Int4>,
        dessert -> Nullable<Int4>,
        cal -> Nullable<Float8>,
        sodium -> Nullable<Float8>,
        sugar -> Nullable<Float8>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    diets,
    dishes,
    meals,
);
