FROM rust:slim-buster
WORKDIR /app
COPY . .
RUN cargo build 
EXPOSE 8080
# --release
CMD ["./target/debug/broker"]