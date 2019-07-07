# select build image
FROM rustlang/rust:nightly-slim as build

# create a new empty shell project
RUN USER=root cargo new --bin pin-go
WORKDIR /pin-go

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/pin-go*
RUN cargo build --release

# our final base
FROM rustlang/rust:nightly-slim

# copy the build artifact from the build stage
COPY --from=build /pin-go/target/release/pin-go .

# set the startup command to run your binary
CMD ["./pin-go"]

