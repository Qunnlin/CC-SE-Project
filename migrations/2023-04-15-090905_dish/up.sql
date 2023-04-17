create table dishes (
    dish_id serial PRIMARY KEY,
    name VARCHAR ( 50 ) UNIQUE NOT NULL,
    calories FLOAT NOT NULL,
    sodium FLOAT NOT NULL,
    sugar FLOAT NOT NULL,
    serving_size FLOAT NOT NULL
);
create table meals (
    meal_id serial PRIMARY KEY,
    name VARCHAR ( 50 ) UNIQUE NOT NULL,
    appetizer INTEGER REFERENCES dishes ( dish_id ),
    entree INTEGER REFERENCES dishes ( dish_id ),
    dessert INTEGER REFERENCES dishes ( dish_id ),
    cal FLOAT,
    sodium FLOAT,
    sugar FLOAT
)