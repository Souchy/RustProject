# FROM rust:1.67 as builder
# WORKDIR /usr/src/myapp
# COPY . .
# RUN cargo install --path .

# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
# CMD ["myapp"]


# Rust as the base image
FROM rust:1.67

# 1. Create a new empty shell project
RUN USER=root cargo new --bin realm
WORKDIR /realm

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/realm*
# RUN cargo build --release
RUN cargo install --path .

CMD ["realm"]
