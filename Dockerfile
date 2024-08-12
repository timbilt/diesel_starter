FROM rust:bullseye as builder

WORKDIR /app/

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

CMD ["cargo", "watch", "--why", "-x", "build"]