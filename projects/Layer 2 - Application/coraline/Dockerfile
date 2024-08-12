# Rust as the base image
FROM rust:latest AS build

# 1. Create empty shell projects
RUN USER=root cargo new --lib "./Layer - Util/teal"
RUN USER=root cargo new --lib "./Layer 0 - Domain/coral-commons"
RUN USER=root cargo new --lib "./Layer 0 - Domain/realm-commons"
RUN USER=root cargo new --bin "./Layer 2 - Application/coraline"

# 2. Copy our manifests
COPY ["./Layer - Util/teal/Cargo.toml", "./Layer - Util/teal/Cargo.toml"]
COPY ["./Layer 0 - Domain/coral-commons/Cargo.toml", "./Layer 0 - Domain/coral-commons/Cargo.toml"]
COPY ["./Layer 0 - Domain/realm-commons/Cargo.toml", "./Layer 0 - Domain/realm-commons/Cargo.toml"]
COPY ["./Layer 2 - Application/coraline/Cargo.toml", "./Layer 2 - Application/coraline/Cargo.toml"]

# 3. Build only the dependencies to cache them
WORKDIR "/Layer 2 - Application/coraline"
RUN cargo build --release
# 3.1 Install Protoc
RUN apt-get update
RUN apt install -y protobuf-compiler

# 4. Remove default sources from shell projects
WORKDIR "/"
RUN rm "/Layer - Util/teal/src/"*.rs
RUN rm "/Layer 0 - Domain/coral-commons/src/"*.rs
RUN rm "/Layer 0 - Domain/realm-commons/src/"*.rs
RUN rm "/Layer 2 - Application/coraline/src/"*.rs

# 5. Copy source code
COPY ["./Layer - Util/teal/src", "./Layer - Util/teal/src"]
COPY ["./Layer - Util/teal/build.rs", "./Layer - Util/teal/build.rs"]
COPY ["./Layer 0 - Domain/coral-commons/src", "./Layer 0 - Domain/coral-commons/src"]
COPY ["./Layer 0 - Domain/coral-commons/build.rs", "./Layer 0 - Domain/coral-commons/build.rs"]
COPY ["./Layer 0 - Domain/realm-commons/src", "./Layer 0 - Domain/realm-commons/src"]
COPY ["./Layer 0 - Domain/realm-commons/build.rs", "./Layer 0 - Domain/realm-commons/build.rs"]
COPY ["./Layer 2 - Application/coraline/src", "./Layer 2 - Application/coraline/src"]

# 6. Build for release.
WORKDIR "/Layer 2 - Application/coraline/"
RUN cargo build --release

# Use a slim Dockerfile with just our app to publish
FROM debian:latest AS app
COPY --from=build ["/Layer 2 - Application/coraline/target/release/coraline", "/coraline"]
COPY ["/Layer 2 - Application/coraline/.env.docker", "/.env.docker"]

ENV ROCKET_ADDRESS=127.0.0.1
ENV ROCKET_PORT=7000
EXPOSE 7000

ENTRYPOINT ["/coraline", ".env.docker"]
# ENTRYPOINT ["cargo", "run", "--release", ".env.docker"]
