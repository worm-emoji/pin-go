# select build image
FROM rustlang/rust:nightly@sha256:c1ad1cbe7bb7ca62eee1dcca62fae6f56b2bd3cfc284a113219cc15916ba7c64 as build

COPY ./ ./

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/pin_go*
RUN cargo build --release

# our final base
FROM rustlang/rust:nightly@sha256:c1ad1cbe7bb7ca62eee1dcca62fae6f56b2bd3cfc284a113219cc15916ba7c64

# copy the build artifact from the build stage
COPY --from=build ./target/release/pin-go .

EXPOSE 8000

# set the startup command to run your binary
CMD ROCKET_PORT=$PORT ./pin-go