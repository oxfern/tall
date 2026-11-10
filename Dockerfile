# syntax=docker/dockerfile:1
FROM rust:1.91.0-bookworm AS build
RUN rustup target add wasm32-unknown-unknown \
 && cargo install wasm-bindgen-cli --version 0.2.129 --locked
WORKDIR /app
COPY . .
# The client (wasm, into web/dist) and the server.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    ./build.sh \
 && cargo build --release -p tall-server \
 && cp target/release/tall /tall

FROM gcr.io/distroless/cc-debian12
COPY --from=build /tall /tall
COPY --from=build /app/web/dist /srv
ENV PORT=8080 STATIC_DIR=/srv
EXPOSE 8080
CMD ["/tall"]
