FROM lukemathwalker/cargo-chef:latest-rust-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release -p data_iceberg --bin data_iceberg

FROM gcr.io/distroless/cc-debian12 AS runtime

COPY --from=builder /app/target/release/data_iceberg /usr/local/bin/data_iceberg

ENTRYPOINT ["/usr/local/bin/data_iceberg"]
