# First step we build the app
# Then we start from a new image to avoid tons of useless tools
FROM rust:1.45 as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/backend/target/release/rust-blog-backend /app/backend
COPY ./src/articles/data /app/data

CMD ["/app/backend"]
