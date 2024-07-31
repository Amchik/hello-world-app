FROM rust:1.79 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:latest

COPY --from=builder /app/target/release/hello-world-app /usr/local/bin/hello-world-app

EXPOSE 3000

CMD ["hello-world-app"]