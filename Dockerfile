FROM rust:alpine as builder
WORKDIR /usr/src/myapp
COPY . .
RUN apk add --no-cache musl-dev pkgconfig mariadb-connector-c-dev openssl-dev sqlite-libs postgresql-libs
RUN cargo install --path .

FROM alpine
RUN apk add --no-cache mariadb-connector-c-dev openssl-dev sqlite-libs postgresql-libs
COPY --from=builder /usr/src/myapp/target/release/server /usr/local/bin/server
COPY --from=builder /usr/src/myapp/target/release/cli /usr/local/bin/cli
CMD ["server"]