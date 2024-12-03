FROM rustlang/rust:nightly-bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
RUN cargo install sqlx-cli@0.8.2 --no-default-features --features native-tls,postgres
COPY . .
# DATABASE_URL is needed to build query_as! macros
RUN --mount=type=secret,id=DATABASE_URL \
    DATABASE_URL="$(cat /run/secrets/DATABASE_URL)" sqlx migrate run \
    && DATABASE_URL="$(cat /run/secrets/DATABASE_URL)" cargo build --release
RUN mv ./target/release/blisk_backend ./app

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt install -y openssl ca-certificates
COPY settings/ ./settings
COPY --from=builder /app/app ./app
ENTRYPOINT ["./app"]