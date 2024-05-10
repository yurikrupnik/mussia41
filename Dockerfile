FROM messense/rust-musl-cross:x86_64-musl AS builder
WORKDIR /app
ARG APP_NAME
COPY . .
RUN cargo build --release -p $APP_NAME --target x86_64-unknown-linux-musl

FROM scratch AS final
COPY --from=builder /app/dist/target/x86_64-unknown-linux-musl/release/$APP_NAME /app
ENV PORT=8080
EXPOSE ${PORT}
ENTRYPOINT ["/app"]