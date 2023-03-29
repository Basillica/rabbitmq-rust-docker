# FROM rust:1.55.0 as builder

# WORKDIR /app

# COPY Cargo.toml Cargo.lock ./
# COPY ./target/debug/broker /app/
# RUN mkdir src \
#  && echo "fn main() {}" > src/main.rs \
#  && cargo build --release

# COPY src/ ./src/
# RUN cargo install --path .

# FROM debian:buster-slim

# RUN apt-get update \
#  && apt-get install -y rabbitmq-server \
#  && rm -rf /var/lib/apt/lists/*

# COPY --from=builder /usr/local/cargo/bin/consumer /usr/local/bin/consumer

# CMD ["consumer"]

FROM rust
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/consumer"]