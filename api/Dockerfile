FROM rust:1 as dev
# Choose a workdir
WORKDIR /app
# Install watch for dev environment
RUN cargo install cargo-watch
# Install diesel for migration
RUN cargo install diesel_cli --no-default-features --features postgres
# Create blank project
RUN USER=root cargo init --bin
# Copy Cargo.toml and Cargo.lock to get dependencies
COPY Cargo.toml .
COPY Cargo.lock .
# This is a dummy build to get the dependencies cached
RUN cargo build
# Remove last build result
RUN rm target/debug/deps/aoj_icpc_api_server-*

FROM dev as build
# This is a dummy build to get the dependencies cached
RUN cargo build --release
# Remove last build result
RUN rm target/release/deps/aoj_icpc_api_server-*
# Copy sources
COPY src src
# Build app (bin will be in /usr/src/app/target/release/aoj-icpc-api-server)
RUN cargo build --release --bin aoj-icpc-api-server

FROM debian:stretch-slim
# Copy bin from builder to this new image
COPY --from=build /app/target/release/aoj-icpc-api-server /bin/
# Default command, run app
CMD ["aoj-icpc-api-server"]
