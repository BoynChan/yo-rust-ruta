# Rust as the base image
FROM rust as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin server
RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev
WORKDIR /server

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./proto ./proto
COPY ./build.rs ./build.rs
COPY ./src ./src

# 5. Build for release.
# RUN rm ./target/release/deps/server*
RUN cargo build --release

# our final base
FROM debian:stable-slim

# copy the build artifact from the build stage
COPY --from=build /server/target/release/server .

# set the startup command to run your binary
CMD ["/server"]