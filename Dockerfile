FROM rust as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian
RUN apt-get update && apt-get dist-upgrade -y && apt-get install -y libssl-dev libsqlite3-dev libpq-dev libmariadb-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/myapp/target/release/server /usr/local/bin/server
COPY --from=builder /usr/src/myapp/target/release/cli /usr/local/bin/cli
CMD ["server"] 