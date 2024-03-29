# This Dockerfile is needed to build the database shell image

# Rust official base image
FROM rust:latest as build
WORKDIR /app

# Only engine and shell needed
RUN mkdir engine
RUN mkdir shell
COPY engine engine
COPY shell shell

# Install protocol buffers compiler
RUN apt-get update && apt-get install -y protobuf-compiler

# Needed to compile into static binary
# https://stackoverflow.com/questions/31770604/how-to-generate-statically-linked-executables/31778003#31778003
RUN rustup target add x86_64-unknown-linux-musl

# Build release binary
RUN cd shell && cargo build --target=x86_64-unknown-linux-musl --release

FROM alpine
WORKDIR /app
COPY --from=build /app/shell/target/x86_64-unknown-linux-musl/release/shell /app
CMD ["./shell"]