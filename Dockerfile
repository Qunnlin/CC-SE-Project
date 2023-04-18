

# Base image with Rust and Cargo installed
FROM rust:1.61.0 AS builder

# Copy the app source code to the container
COPY . .

# Start with a fresh image
FROM debian:buster-slim

# Install PostgreSQL and its client library
RUN apt-get update \
    && apt-get install -y postgresql postgresql-client \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*


# Copy the app binary from the previous build stage to the container
COPY --from=builder ./target/release/meals_api .

# Expose the default port used by the app
EXPOSE 8000

# Set environment variables for the PostgreSQL database
ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD password
ENV POSTGRES_DB dishes

# Run the PostgreSQL database as a background process
CMD ["pg_ctlcluster", "11", "main", "start"]


# Run the diesel CLI to generate the database schema
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel migration run

# Build the Rust app
RUN cargo build --release

# Start the Rust app with the PostgreSQL database
ENTRYPOINT ["./target/release/meals_api", "--postgres-host=127.0.0.1", "--postgres-port=5432", "--postgres-user=postgres", "--postgres-password=password", "--postgres-db=dishes"]
