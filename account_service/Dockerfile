FROM rust:1.60

ENV CARGO_TERM_COLOR always
# Update User's system, install postgres into users system 
RUN apt-get update && \
    apt-get -y upgrade && \
    # Check argonautica's specification
    apt-get -y install libpq-dev clang  \
    llvm-dev libclang-dev openssl cmake 
# RUN cargo install sqlx-cli --no-default-features --features postgres
# RUN cargo sqlx prepare -- --lib
# create empty project for caching dependencies
RUN USER=root cargo new --bin /account_service/docker-build

COPY /common_utils/ ./account_service/common_utils/

WORKDIR /account_service/docker-build

COPY /Cargo.lock ./

COPY /account_service/Cargo.toml ./
# cache dependencies
# RUN cargo install sqlx-cli --no-default-features --features postgres
# RUN cargo sqlx prepare -- --lib
RUN cargo install --path . --locked
# RUN cargo build --release

COPY /account_service/ ./

# RUN cargo install sqlx-cli --no-default-features --features postgres
# RUN cargo sqlx prepare -- --lib
RUN cargo install --path . --locked
# RUN cargo build --release


FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get -y upgrade && \
    # Check argonautica's specification
    apt-get -y install libpq-dev clang \
    llvm-dev libclang-dev openssl cmake 

COPY --from=0 /usr/local/cargo/bin/account_service /usr/local/bin/account_service

CMD ["account_service"]