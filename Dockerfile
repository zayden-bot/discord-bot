FROM rust:latest as builder

WORKDIR /usr/src/zayden
COPY . .

RUN cargo build --release --bin zayden

FROM debian:bookworm-slim AS runtime

COPY --from=builder /usr/src/zayden/target/release/zayden /usr/local/bin/
COPY .env /usr/local/bin/
COPY good_morning /usr/local/bin/good_morning
COPY good_night /usr/local/bin/good_night

RUN apt-get update && apt-get install -y openssl && apt clean && rm -rf /var/lib/apt/lists/*

EXPOSE 80

WORKDIR /usr/local/bin

CMD ["zayden"]