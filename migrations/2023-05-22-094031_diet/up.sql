create table diets (
    id serial PRIMARY KEY,
    name varchar(255) NOT NULL,
    cal int NOT NULL,
    sodium int NOT NULL,
    sugar int NOT NULL
);