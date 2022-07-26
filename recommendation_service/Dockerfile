FROM rust:1.60

ENV CARGO_TERM_COLOR always
# Update User's system, install postgres into users system 
RUN apt-get update && \
    apt-get -y upgrade && \
    # Check argonautica's specification
    apt-get -y install libpq-dev clang  \
    llvm-dev libclang-dev openssl cmake 
# create empty project for caching dependencies
RUN USER=root cargo new --bin /recommendation_service/docker-build

COPY /common_utils/ ./recommendation_service/common_utils/

WORKDIR /recommendation_service/docker-build

COPY /Cargo.lock ./

COPY /recommendation_service/Cargo.toml ./
# cache dependencies
RUN cargo install --path . --locked
# RUN cargo build --release

COPY /recommendation_service/ ./

RUN cargo install --path . --locked
# RUN cargo build --release


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev curl

COPY --from=0 /usr/local/cargo/bin/recommendation_service /usr/local/bin/recommendation_service

CMD ["recommendation_service"]