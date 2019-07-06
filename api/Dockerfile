FROM rust:1 as dev
# Choose a workdir
WORKDIR /app
# Install watch for dev environment
RUN cargo install cargo-watch
# Install diesel for migration
RUN cargo install diesel_cli --no-default-features --features postgres
# Create blank project
RUN USER=root cargo init --bin
# Create empty lib for dummy build
RUN touch src/lib.rs
# Copy Cargo.toml and Cargo.lock to get dependencies
COPY Cargo.toml .
COPY Cargo.lock .
# This is a dummy build to get the dependencies cached
RUN cargo build
# Remove last build result
RUN rm target/debug/deps/*aoj_icpc*

FROM dev as build
# This is a dummy build to get the dependencies cached
RUN cargo build --release
# Remove last build result
RUN rm target/release/deps/*aoj_icpc*
# Copy sources
COPY src src
# Build app (bin will be in /usr/src/app/target/release/aoj-icpc-api-server)
RUN cargo build --release --bin aoj-icpc-api-server
# Build crawler (TODO: Separete into different container)
RUN cargo build --release --bin aoj_crawler

FROM debian:stretch-slim
# Install dependencies
RUN apt-get update && apt-get install -y libpq-dev
# Copy app from builder to this new image
COPY --from=build /app/target/release/aoj-icpc-api-server /bin/
# Copy crawler from builder to this new image
COPY --from=build /app/target/release/aoj_crawler /bin/
# Default command, run app
CMD ["aoj-icpc-api-server", "--port", "80"]
