FROM rust

WORKDIR /app

COPY . /app

RUN cargo build

EXPOSE 8080

RUN cargo run