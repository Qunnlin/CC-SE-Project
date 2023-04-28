FROM postgres:latest as db

ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD password
ENV POSTGRES_DB dishes


FROM rust:slim as app
#
WORKDIR /usr/src/app
COPY . .

# Dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    default-libmysqlclient-dev \
    && rm -rf /var/lib/apt/lists/*



RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

RUN diesel setup && diesel migration run && cargo build --release

EXPOSE 8000

CMD cargo run --release