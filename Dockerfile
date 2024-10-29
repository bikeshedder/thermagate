#
# Stage 1: Build the sources
#
FROM rust:1.72-alpine AS build

RUN apk add musl-dev

WORKDIR /repo

# Only build the dependencies
COPY Cargo.toml Cargo.lock ./
RUN \
    mkdir /repo/src && \
    echo 'fn main() {}' > /repo/src/main.rs && \
    cargo build --release && \
    rm -Rvf /repo/src

# Build the rest of the code
COPY src ./src
RUN \
     cargo install --frozen --offline --root /build --path .

#
# Stage 2: Copy the binaries into a slim container
#
FROM alpine AS rt
COPY --from=build /build/bin/* /bin/

CMD ["/bin/thermagate"]
