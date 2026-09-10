FROM lukemathwalker/cargo-chef:latest-rust-1-trixie AS chef
WORKDIR /nameless

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /nameless/recipe.json recipe.json
RUN apt-get update && apt-get install -y build-essential autoconf automake libtool m4 libopus-dev
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:trixie AS final
WORKDIR /nameless

COPY --from=builder /nameless/target/release/nameless-ng .

CMD ["/nameless/nameless-ng"]
