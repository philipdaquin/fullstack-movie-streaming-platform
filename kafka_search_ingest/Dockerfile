FROM rust:1.60

ENV CARGO_TERM_COLOR always
# Update User's system, install postgres into users system 
RUN apt-get update && \
    apt-get -y upgrade && \
    # Check argonautica's specification
    apt-get -y install libpq-dev clang  \
    llvm-dev libclang-dev openssl cmake 

# create empty project for caching dependencies
RUN USER=root cargo new --bin /kafka_search_ingest/docker-build

COPY /common_utils/ ./kafka_search_ingest/common_utils/

WORKDIR /kafka_search_ingest/docker-build

COPY /Cargo.lock ./

COPY /kafka_search_ingest/Cargo.toml ./
# cache dependencies
RUN cargo install --path . --locked
# RUN cargo build --release

COPY /kafka_search_ingest/ ./

RUN cargo install --path . --locked
# RUN cargo build --release


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev curl

COPY --from=0 /usr/local/cargo/bin/kafka_search_ingest /usr/local/bin/kafka_search_ingest

CMD ["kafka_search_ingest"]