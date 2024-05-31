FROM rust:1 AS chef
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM messense/rust-musl-cross:x86_64-musl AS builder
WORKDIR /app
ARG APP_NAME
COPY --from=planner /app/recipe.json recipe.json
RUN cargo install cargo-chef --locked
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release -p $APP_NAME --target x86_64-unknown-linux-musl

FROM scratch AS final
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/$APP_NAME /app
ENV PORT=8080
EXPOSE ${PORT}
ENTRYPOINT ["/app"]