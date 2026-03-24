FROM rust:trixie AS base

ENV USER=root
ENV SQLX_OFFLINE=true
# Some tower/axum variables?

WORKDIR /app
RUN cargo init
COPY Cargo.toml /app/Cargo.toml
RUN cargo fetch
COPY . /app
COPY .sqlx/ /app/.sqlx

FROM base AS development

CMD ["cargo", "run", "--offline"]

FROM base as builder

RUN cargo build --release --offline

FROM debian:bookworm-slim as production

RUN useradd -m appuser
USER appuser

COPY --from=builder /app/target/release/Roombooker /Roombooker
COPY static/ /static

EXPOSE 3000

CMD ["/Roombooker"]
