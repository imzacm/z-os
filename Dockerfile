FROM ubuntu:latest AS base
RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y \
    build-essential bison flex libgmp3-dev \
    libmpc-dev libmpfr-dev texinfo \
    libcloog-isl-dev libisl-dev \
COPY ./setup-toolchain.sh /tmp/setup-toolchain.sh
RUN chmod +x /tmp/setup-toolchain.sh && /tmp/setup-toolchain.sh

FROM base AS chef
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release
