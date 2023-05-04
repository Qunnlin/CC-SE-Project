# Cloud Computing & Software Engineering Assignment 1

Assignment 1 of the Cloud Computing & software Engineering Class at the Reichman University, Herzliya.

## Description

REST API for a meal creation service.
The API allows users to create dishes and meals. The Service uses the Ninjas API to retrieve nutritional information for the dishes.

The API is implemented in Rust using the Actix Web framework and Diesel for the database.

## Getting Started

 ### Docker
The easiest way to run the service is to use the provided Dockerfile.
To build the image run:
```bash
docker build -t meals_api:v1 .
```

Since this version of the API already uses a Database but a single Dockerfile for submission is required, the Dockerfile installs Postgres and runs the migrations on startup.
This solutions is a bit "hacky" but it works for the purpose of this assignment.
The starttup script is located in the install.sh file.

To run the image run:
```bash
docker run -p 8000:8000 meals_api:v1
```

### Local
To run the service locally you need to have a Postgres database running.
The database connection string can be configured in the .env file.
To run the service run:

### Modification

- To change the port the service is running on change the port in the main.rs file.
- To change the database connection string change the DATABASE_URL in the .env file.
- To change the API key for the Ninjas API change the NINJA_API_KEY in the .env file.

## Author

* **Felix Schick** - [email](mailto:felixsteffen.schick@post.runi.ac.il)

## References

### Assignment specification:
- [Assignment 1 Specification](./assignment1_requirements.pdf)

### Used Crates and External APIs:
- [Ninjas Nutrition API](https://api-ninjas.com/api/nutrition)
- [Actix Web](https://actix.rs/)
- [Diesel](https://diesel.rs/)


