FROM rust:1.72-alpine

RUN apk add musl-dev

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["altherma-gateway"]
