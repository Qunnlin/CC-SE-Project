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

EXPOSE 8000

CMD ./install.sh