# # Rust as the base image
# FROM rust:1.49

# # 1. Create a new empty shell project
# RUN USER=root cargo new --bin producer
# WORKDIR /producer

# # 2. Copy our manifests
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml

# # 3. Build only the dependencies to cache them
# RUN cargo build

# # 4. Now that the dependency is built, copy your source code
# COPY . .

# # 5. Build for release.
# RUN rm ./target/release/deps/producer*
# RUN cargo install --path .

# CMD ["producer"]


# FROM rust:1.55.0 as builder

# WORKDIR /app

# COPY Cargo.toml Cargo.lock ./
# RUN mkdir src \
#  && echo "fn main() {}" > src/main.rs \
#  && cargo build --release

# COPY src/ ./src/
# RUN cargo install --path .

# FROM debian:buster-slim

# RUN apt-get update \
#  && apt-get install -y rabbitmq-server \
#  && rm -rf /var/lib/apt/lists/*

# COPY --from=builder /usr/local/cargo/bin/producer /usr/local/bin/producer

# CMD ["producer"]


FROM rust
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/producer"]
