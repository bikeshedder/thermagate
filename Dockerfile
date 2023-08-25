FROM rust:1.72-alpine AS build

RUN apk add musl-dev

WORKDIR /src
COPY . .

RUN cargo install --path . --root /build


FROM rust:1.72-alpine AS rt
COPY --from=build /build/bin/* /bin/

CMD ["/bin/altherma-gateway"]
