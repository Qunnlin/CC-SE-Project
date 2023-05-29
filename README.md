# Cloud Computing & Software Engineering Project

Project of the Cloud Computing & software Engineering Class at the Reichman University, Herzliya.

## Description

This Repository holds the code for the project of the Cloud Computing & Software Engineering Class at the Reichman University, Herzliya.  
The Project is starts as a simple REST API for a nutrition app, which is then extended with a database and deployed with docker.  
Additionally, a reverse proxy is added to the deployment to allow load balancing.

- The REST API is written in **Rust** using the **Actix** Web Framework and the **Diesel** ORM.
- A **Postgres** Database is used to store the data.
- **HAProxy** is used as a reverse proxy and load balancer.
- To improve docker build times, **cargo-chef** is used to cache the dependencies.


## Getting Started

### Prerequisites

Since the project is deployed with docker, the only prerequisite is a working docker installation.  
For instructions on how to install docker, see the [official documentation](https://docs.docker.com/get-docker/).

### Installing

To install the project, simply clone the repository and run the docker-compose file:

```bash
docker compose up
```

This will build the docker images and start the containers.
By default, the API will be available at [http://localhost:80](http://localhost:80).

To stop the containers, run:
```bash
docker compose down
```


## Author

* **Felix Schick** - [email](mailto:felixsteffen.schick@post.runi.ac.il)

## References


### Assignment specifications:
- [Assignment 1 Specification](specification/assignment1_requirements.pdf)
- [Assignment 2 Specification](specification/assignment2_requirements.pdf)


### Used Crates and External APIs:
- [Ninjas Nutrition API](https://api-ninjas.com/api/nutrition)
- [Actix Web](https://actix.rs/)
- [Diesel](https://diesel.rs/)

### Other References:
- [Docker](https://www.docker.com/)
- [HAProxy](https://www.haproxy.org/)
- [Cargo Chef](https://www.lpalmieri.com/posts/fast-rust-docker-builds/)
- [PostgreSQL](https://www.postgresql.org/)



