# FROM rust:1.60

# ENV CARGO_TERM_COLOR always
# # Update User's system, install postgres into users system 
# RUN apt-get update && \
#     apt-get -y upgrade && \
#     # Check argonautica's specification
#     apt-get -y install libpq-dev clang  \
#     llvm-dev libclang-dev openssl cmake 
# # RUN cargo install sqlx-cli --no-default-features --features postgres
# # RUN cargo sqlx prepare -- --lib
# # create empty project for caching dependencies
# RUN USER=root cargo new --bin /account_service/docker-build

# COPY /common_utils/ ./account_service/common_utils/

# WORKDIR /account_service/docker-build

# COPY /Cargo.lock ./

# COPY /account_service/Cargo.toml ./
# # cache dependencies
# # RUN cargo install sqlx-cli --no-default-features --features postgres
# # RUN cargo sqlx prepare -- --lib
# RUN cargo install --path . --locked
# # RUN cargo build --release

# COPY /account_service/ ./

# # RUN cargo install sqlx-cli --no-default-features --features postgres
# # RUN cargo sqlx prepare -- --lib
# RUN cargo install --path . --locked
# # RUN cargo build --release


# FROM debian:bookworm-slim

# RUN apt-get update && \
#     apt-get -y upgrade && \
#     # Check argonautica's specification
#     apt-get -y install libpq-dev clang \
#     llvm-dev libclang-dev openssl cmake 

# COPY --from=0 /usr/local/cargo/bin/account_service /usr/local/bin/account_service

# CMD ["account_service"]



FROM lukemathwalker/cargo-chef:latest-rust-1.72.0 as chef
WORKDIR /app
RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install pkg-config && \
  apt-get -y install libssl-dev  && \
  apt-get install -y libclang-dev && \
  apt-get install -y cmake  && \
  apt-get install -y build-essential && \
  apt-get clean
FROM chef as planner
COPY . .
COPY models /app/models/

# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
COPY models /app/models

# ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin account_service
FROM debian:bookworm-slim AS runtime

WORKDIR /app
COPY models /app/models
# COPY . . 
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates pkg-config libssl-dev libclang-dev cmake build-essential \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/account_service account_service
# COPY configuration configuration
# ENV APP_ENVIRONMENT production
EXPOSE 4001
ENTRYPOINT ["./account_service"]

