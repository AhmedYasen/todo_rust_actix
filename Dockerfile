FROM rust:1.42 as builder
WORKDIR /usr/src/myapp
COPY . .

# RUN rustup update
RUN rustup default nightly

RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install && rm -rf /var/lib/apt/lists/*

EXPOSE 8811

WORKDIR /usr/local/bin/myapp

COPY --from=builder /usr/src/myapp/target/release/todo_rust_actix /usr/local/bin/myapp/todo_rust_actix
COPY --from=builder /usr/src/myapp/config.toml ./config.toml

CMD ["./todo_rust_actix"]