FROM rust:latest

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN apt-get update
RUN apt-get install curl
RUN curl -s https://packagecloud.io/install/repositories/ookla/speedtest-cli/script.deb.sh | bash
RUN apt-get install speedtest

COPY ./speedtest.lisence.json /root/.config/ookla/speedtest-cli.json

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/inet-mon"]