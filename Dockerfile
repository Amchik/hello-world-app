FROM rust:1-alpine as builder

WORKDIR /app

COPY . .

RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine:latest

COPY --from=builder /app/target/release/hello-world-app /usr/local/bin/hello-world-app

EXPOSE 3000

CMD ["hello-world-app"]
