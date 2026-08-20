FROM lukemathwalker/cargo-chef:latest-rust-1.97.1-trixie AS chef
WORKDIR /nameless

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /nameless/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=1
RUN cargo build --release

FROM debian:trixie AS final
WORKDIR /nameless

COPY --from=builder /nameless/target/release/nameless-ng .

CMD ["/nameless/nameless-ng"]
