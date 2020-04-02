FROM rust:1.40.0-slim-stretch

RUN cargo install cargo-watch

RUN USER=root cargo new app
WORKDIR /app
COPY ./Cargo.toml Cargo.toml
#COPY ./Cargo.lock Cargo.lock

COPY . /app
RUN cargo build

CMD ["cargo", "run"]
