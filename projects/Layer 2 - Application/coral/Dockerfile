# Rust as the base image
FROM rust:1.70-slim-buster

# 1. Create empty shell projects
RUN USER=root cargo new --lib "./Layer - Util/teal"
RUN USER=root cargo new --lib "./Layer 0 - Domain/eevee"
RUN USER=root cargo new --lib "./Layer 0 - Domain/coral-commons"
RUN USER=root cargo new --lib "./Layer 0 - Domain/realm-commons"
RUN USER=root cargo new --bin "./Layer 2 - Application/coral"

# 2. Copy our manifests
COPY ["./Layer - Util/teal/Cargo.toml", "./Layer - Util/teal/Cargo.toml"]
COPY ["./Layer 0 - Domain/eevee/Cargo.toml", "./Layer 0 - Domain/eevee/Cargo.toml"]
COPY ["./Layer 0 - Domain/coral-commons/Cargo.toml", "./Layer 0 - Domain/coral-commons/Cargo.toml"]
COPY ["./Layer 0 - Domain/realm-commons/Cargo.toml", "./Layer 0 - Domain/realm-commons/Cargo.toml"]
COPY ["./Layer 2 - Application/coral/Cargo.toml", "./Layer 2 - Application/coral/Cargo.toml"]

# 3. Build only the dependencies to cache them
WORKDIR "/Layer 2 - Application/coral"
RUN cargo build --release

# 4. Remove default sources from shell projects
WORKDIR "/"
RUN rm "/Layer - Util/teal/src/"*.rs
RUN rm "/Layer 0 - Domain/eevee/src/"*.rs
RUN rm "/Layer 0 - Domain/coral-commons/src/"*.rs
RUN rm "/Layer 0 - Domain/realm-commons/src/"*.rs
RUN rm "/Layer 2 - Application/coral/src/"*.rs

# 5. Copy source code
COPY ["./Layer - Util/teal/src", "./Layer - Util/teal/src"]
COPY ["./Layer - Util/teal/build.rs", "./Layer - Util/teal/build.rs"]
COPY ["./Layer 0 - Domain/eevee/src", "./Layer 0 - Domain/eevee/src"]
COPY ["./Layer 0 - Domain/coral-commons/src", "./Layer 0 - Domain/coral-commons/src"]
COPY ["./Layer 0 - Domain/coral-commons/build.rs", "./Layer 0 - Domain/coral-commons/build.rs"]
COPY ["./Layer 0 - Domain/realm-commons/src", "./Layer 0 - Domain/realm-commons/src"]
COPY ["./Layer 0 - Domain/realm-commons/build.rs", "./Layer 0 - Domain/realm-commons/build.rs"]
COPY ["./Layer 2 - Application/coral/src", "./Layer 2 - Application/coral/src"]

# 6. Build for release.
WORKDIR "/Layer 2 - Application/coral/"
RUN cargo build --release

EXPOSE 8080
ENTRYPOINT ["cargo", "run"]
