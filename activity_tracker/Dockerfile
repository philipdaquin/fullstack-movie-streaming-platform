FROM rust:1.60

ENV CARGO_TERM_COLOR always
# Update User's system, install postgres into users system 
RUN apt-get update && \
    apt-get -y upgrade && \
    # Check argonautica's specification
    apt-get -y install libpq-dev clang cmake  

# create empty project for caching dependencies
RUN USER=root cargo new --bin /activity_tracker/docker-build

COPY /common_utils/ ./activity_tracker/common_utils/

WORKDIR /activity_tracker/docker-build

COPY /Cargo.lock ./

COPY /activity_tracker/Cargo.toml ./
# cache dependencies
RUN cargo install --path . --locked
# RUN cargo build --release

COPY /activity_tracker/ ./

RUN cargo install --path . --locked
# RUN cargo build --release


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev curl

COPY --from=0 /usr/local/cargo/bin/activity_tracker /usr/local/bin/activity_tracker

CMD ["activity_tracker"]