# Use an official Rust runtime as a parent image
FROM rust:latest as builder 

WORKDIR /app

COPY src/ ./src/
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

# Final base
FROM rockylinux:9.2-minimal
WORKDIR /app
COPY --from=builder /app/target/release/rust-monitoring .
RUN chmod +x ./rust-monitoring
CMD while true; do ./rust-monitoring; sleep 10; done