# This File Tells Docker how to Build the app into image

# Step-1 Using the Rust offical image to Build
FROM rust:latest AS builder

WORKDIR /app

# install system dependenices needed for compile
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*


# copy dependency file
# this way docker caches dependencies
# only redownload if Cargo.toml changes
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# now copy the actual source code
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

ENV SQLX_OFFLINE=true

# build the real binary 
RUN touch src/main.rs && cargo build --release

# Step-2 smaller image just for running
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy only the compiled  binary from step-1
COPY --from=builder /app/target/release/iam_platform .
COPY --from=builder /app/migrations ./migrations

EXPOSE 3000

CMD [ "./iam_platform" ]