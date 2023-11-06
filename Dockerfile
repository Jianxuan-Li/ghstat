FROM rust:1.73.0 as builder

WORKDIR /usr/src/ghstat
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo install --path .

FROM ubuntu:latest

RUN apt-get update \
    & apt-get install -y extra-runtime-dependencies \
    & rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/ghstat /usr/local/bin/ghstat

CMD ["ghstat"]
