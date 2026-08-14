FROM rust:1.97-bookworm AS builder
COPY ./migrations ./migrations
COPY ./src/ ./src
COPY ./crates/ ./crates
COPY nameless.toml diesel.toml Cargo.toml Cargo.lock ./
RUN cargo build --release

FROM debian:bookworm-slim
COPY nameless.toml /usr/bin/nameless.toml
COPY --from=builder ./target/release/nameless-ng /usr/bin/nameless-ng
USER nameless
CMD ["nameless-ng"]
