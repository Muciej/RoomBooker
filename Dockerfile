FROM rust as builder

WORKDIR /app

# Cache dependencies
COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build real app
COPY ./ ./
RUN cargo build --release

# Temporary until 2nd step is working properly
EXPOSE 3000
CMD ["/app/target/release/backend"]


# FROM debian:bookworm-slim

# RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# WORKDIR /app

# COPY --from=builder /app/target/release/backend /app/backend # doesn't work for some reason
# checked and the path is valid during builder step but for some reason /app/backend is not present afterwards

# EXPOSE 3000

# CMD ["/app/target/release/backend"]
