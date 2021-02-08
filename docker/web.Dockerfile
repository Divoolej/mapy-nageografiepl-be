FROM rust:alpine AS builder

RUN apk add --update --no-cache musl-dev openssl-dev postgresql-dev

WORKDIR /app

COPY Cargo* .

COPY docker/stubs/db.toml db/Cargo.toml
COPY docker/stubs/app.toml app/Cargo.toml
COPY docker/stubs/web.toml web/Cargo.toml

COPY docker/stubs/main.rs db/src/main.rs
COPY docker/stubs/main.rs app/src/main.rs
COPY docker/stubs/main.rs web/src/main.rs

COPY db/Cargo* db/
RUN cargo build --release -p db

COPY app/Cargo* app/
RUN cargo build --release -p app

COPY web/Cargo* web/
RUN cargo build --release -p web

COPY . .
RUN cargo build --release --locked -p db
RUN cargo build --release --locked -p app
RUN cargo build --release --locked -p web

FROM rust:alpine AS runner

RUN apk add --update --no-cache dumb-init

COPY --from=builder /app/target/release/web /app/web

ENTRYPOINT ["/usr/bin/dumb-init", "--"]

EXPOSE 3000

CMD ["/app/web"]
