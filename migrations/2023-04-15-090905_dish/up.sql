create table dishes (
    id serial PRIMARY KEY,
    name VARCHAR ( 50 ) UNIQUE NOT NULL,
    cal FLOAT NOT NULL,
    sodium FLOAT NOT NULL,
    sugar FLOAT NOT NULL,
    size FLOAT NOT NULL
);
create table meals (
    id serial PRIMARY KEY,
    name VARCHAR ( 50 ) UNIQUE NOT NULL,
    appetizer INTEGER REFERENCES dishes ( ID ) ON DELETE SET NULL,
    main INTEGER REFERENCES dishes ( ID ) ON DELETE SET NULL ,
    dessert INTEGER REFERENCES dishes ( ID ) ON DELETE SET NULL,
    cal FLOAT,
    sodium FLOAT,
    sugar FLOAT
);

-- Function to update meal nutrition based on dishes
CREATE OR REPLACE FUNCTION update_meal_nutrition() RETURNS TRIGGER AS $$
BEGIN
    NEW.cal := COALESCE((SELECT cal FROM dishes WHERE id = NEW.appetizer), 0)
        + COALESCE((SELECT cal FROM dishes WHERE id = NEW.main), 0)
        + COALESCE((SELECT cal FROM dishes WHERE id = NEW.dessert), 0);
    NEW.sodium := COALESCE((SELECT sodium FROM dishes WHERE id = NEW.appetizer), 0)
        + COALESCE((SELECT sodium FROM dishes WHERE id = NEW.main), 0)
        + COALESCE((SELECT sodium FROM dishes WHERE id = NEW.dessert), 0);
    NEW.sugar := COALESCE((SELECT sugar FROM dishes WHERE id = NEW.appetizer), 0)
        + COALESCE((SELECT sugar FROM dishes WHERE id = NEW.main), 0)
        + COALESCE((SELECT sugar FROM dishes WHERE id = NEW.dessert), 0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to update meal nutrition
CREATE TRIGGER update_meal_nutrition_trigger
    BEFORE INSERT OR UPDATE ON meals
    FOR EACH ROW
EXECUTE FUNCTION update_meal_nutrition();