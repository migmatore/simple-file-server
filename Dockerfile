FROM rust:1.65.0

WORKDIR /app

COPY src src
COPY Cargo.toml .

RUN cargo build --release

# CMD ["target/release/rust-file-server"]

