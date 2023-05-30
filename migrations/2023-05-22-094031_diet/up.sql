create table diets (
    id serial PRIMARY KEY,
    name varchar(255) NOT NULL,
    cal FLOAT NOT NULL,
    sodium FLOAT NOT NULL,
    sugar FLOAT NOT NULL
);

