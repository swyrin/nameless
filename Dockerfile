FROM lukemathwalker/cargo-chef:latest-rust-1.97.1-trixie AS chef
WORKDIR /nameless

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /nameless/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:trixie AS final
WORKDIR /nameless

COPY --from=builder /nameless/target/release/nameless-ng .

LABEL org.opencontainers.image.authors="swyrin"
LABEL org.opencontainers.image.source=https://github.com/swyrin/nameless
LABEL org.opencontainers.image.description="A Discord bot"
LABEL org.opencontainers.image.licenses=AGPL-3.0-or-later

CMD ["/nameless/nameless-ng"]
