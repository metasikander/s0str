# --- Builder -----------------------------------------------------------------
FROM rust:alpine as builder

# - Build cache ---
run apk upgrade \
    && apk add libc-dev

RUN cargo new --bin /usr/src/s0str
WORKDIR /usr/src/s0str

COPY entity ./entity
COPY migration ./migration
COPY Cargo.* ./

RUN cargo build --release \
   && rm target/release/* -rf

# - Compile ---
COPY ./src/ ./src/
RUN cargo build --release

# --- Executioner -------------------------------------------------------------
FROM alpine:latest

COPY --from=builder /usr/src/s0str/target/release/s0str /usr/local/bin/

CMD /usr/local/bin/s0str