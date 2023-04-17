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
);

CREATE OR REPLACE FUNCTION update_meal_nutrition() RETURNS TRIGGER AS $$
BEGIN
    NEW.cal := COALESCE((SELECT calories FROM dishes WHERE dish_id = NEW.appetizer), 0)
        + COALESCE((SELECT calories FROM dishes WHERE dish_id = NEW.entree), 0)
        + COALESCE((SELECT calories FROM dishes WHERE dish_id = NEW.dessert), 0);
    NEW.sodium := COALESCE((SELECT sodium FROM dishes WHERE dish_id = NEW.appetizer), 0)
        + COALESCE((SELECT sodium FROM dishes WHERE dish_id = NEW.entree), 0)
        + COALESCE((SELECT sodium FROM dishes WHERE dish_id = NEW.dessert), 0);
    NEW.sugar := COALESCE((SELECT sugar FROM dishes WHERE dish_id = NEW.appetizer), 0)
        + COALESCE((SELECT sugar FROM dishes WHERE dish_id = NEW.entree), 0)
        + COALESCE((SELECT sugar FROM dishes WHERE dish_id = NEW.dessert), 0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_meal_nutrition_trigger
    BEFORE INSERT OR UPDATE ON meals
    FOR EACH ROW
EXECUTE FUNCTION update_meal_nutrition();
