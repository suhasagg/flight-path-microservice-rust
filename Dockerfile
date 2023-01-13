# Start with a rust alpine image
FROM rust:1-alpine3.15 as builder
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk update && apk add --no-cache --virtual .build-dependencies \
    cargo \
    build-base \
    file \
    libgcc \
    musl-dev \
    rust \
    openssl-dev \
    llvm-libunwind \
    pkgconfig
# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app
# do a release build
RUN cargo build --release

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.15
# if needed, install additional dependencies here
RUN apk update && apk add --no-cache openssl-dev \
    llvm-libunwind \
    libgcc
# copy the binary into the final image
COPY --from=builder /app/target/release/flight-microservice ./
EXPOSE 8080
# set the binary as entrypoint
CMD ["/flight-microservice"]
