FROM rust:1.60

ENV CARGO_TERM_COLOR always
# Update User's system, install postgres into users system 
RUN apt-get update && \
    apt-get -y upgrade && \
    # Check argonautica's specification
    apt-get -y install libpq-dev clang  \
    llvm-dev libclang-dev openssl cmake 

# create empty project for caching dependencies
RUN USER=root cargo new --bin /tensorflow_consumer/docker-build

COPY /common_utils/ ./tensorflow_consumer/common_utils/

WORKDIR /tensorflow_consumer/docker-build

COPY /Cargo.lock ./

COPY /tensorflow_consumer/Cargo.toml ./
# cache dependencies
RUN cargo install --path . --locked
# RUN cargo build --release

COPY /tensorflow_consumer/ ./

RUN cargo install --path . --locked
# RUN cargo build --release


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev curl

COPY --from=0 /usr/local/cargo/bin/tensorflow_consumer /usr/local/bin/tensorflow_consumer

CMD ["tensorflow_consumer"]