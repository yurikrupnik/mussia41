FROM messense/rust-musl-cross:x86_64-musl AS builder
#FROM rust:1 AS builder
WORKDIR /
ARG APP_NAME
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./nx.json .
#COPY ./apps/rust/actix_app ./apps/rust/actix_app
COPY ./apps ./apps
COPY ./libs ./libs
RUN cargo build --release -p $APP_NAME --target x86_64-unknown-linux-musl
#RUN cargo build --release -p $APP_NAME

FROM scratch AS final
ARG APP_NAME
COPY --from=builder /target/x86_64-unknown-linux-musl/release/$APP_NAME  /app
#COPY --from=builder /target/release/$APP_NAME /app
ENV PORT=8080
EXPOSE ${PORT}
ENTRYPOINT ["/app"]

#FROM messense/rust-musl-cross:x86_64-musl AS builder
#WORKDIR /app
#ARG APP_NAME
#COPY . .
#RUN cargo build --release -p $APP_NAME --target x86_64-unknown-linux-musl
#
#FROM scratch AS final
#COPY --from=builder /app/dist/target/x86_64-unknown-linux-musl/release/$APP_NAME /app
#ENV PORT=8080
#EXPOSE ${PORT}
#ENTRYPOINT ["/app"]