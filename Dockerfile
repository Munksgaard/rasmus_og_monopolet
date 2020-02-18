# rust 1.41.0
FROM rust:1.41.0 as build

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/rasmus_og_monopolet /build-out/

# Ubuntu 18.04
FROM ubuntu@sha256:5f4bdc3467537cbbe563e80db2c3ec95d548a9145d64453b06939c4592d67b6d

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev libpq5 libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/rasmus_og_monopolet /

CMD ["/rasmus_og_monopolet"]
