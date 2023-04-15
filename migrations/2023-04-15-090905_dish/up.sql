create table dishes (
    dish_id serial PRIMARY KEY,
    name VARCHAR ( 50 ) UNIQUE NOT NULL,
    calories FLOAT NOT NULL,
    sodium FLOAT NOT NULL,
    sugar FLOAT NOT NULL,
    serving_size FLOAT NOT NULL
)