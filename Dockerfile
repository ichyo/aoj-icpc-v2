# Copied from https://blog.jawg.io/docker-multi-stage-build/
FROM rust:1 as builder
# Choose a workdir
WORKDIR /usr/src/app
# Create blank project
RUN USER=root cargo init --bin
# Copy Cargo.toml and Cargo.lock to get dependencies
COPY Cargo.toml .
COPY Cargo.lock .
# This is a dummy build to get the dependencies cached
RUN cargo build --release
# Remove last build result
RUN rm target/release/deps/aoj_icpc_v2-*
# Copy sources
COPY src src
# Build app (bin will be in /usr/src/app/target/release/aoj-icpc-v2)
RUN cargo build --release


FROM debian:stretch-slim
# Copy bin from builder to this new image
COPY --from=builder /usr/src/app/target/release/aoj-icpc-v2 /bin/
# Default command, run app
CMD ["aoj-icpc-v2"]
