FROM rust:latest

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/inet-mon"]