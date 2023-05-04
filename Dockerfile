FROM rust:slim as app
#
WORKDIR /usr/src/app
COPY src ./src
COPY migrations ./migrations
COPY Cargo.toml .
COPY diesel.toml .
COPY install.sh .
COPY .env .


# Dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    default-libmysqlclient-dev \
    postgresql-client \
    postgresql \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

EXPOSE 8000

CMD ./install.sh