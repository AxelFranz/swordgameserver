FROM docker.io/lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /opt/swordgameserver

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /opt/swordgameserver/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release


FROM debian:bookworm-slim AS runtime
COPY --from=builder /opt/swordgameserver/target/release/swordgameserver /bin/swordgameserver
WORKDIR /opt/swordgameserver

USER nobody
CMD ["/bin/swordgameserver"]