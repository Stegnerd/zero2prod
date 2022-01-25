FROM lukemathwalker/cargo-chef:latest-rust-1.58.1 as chef
WORKDIR /app

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

# Builder stage
# this means we still have a link to the std library
# does not contribute to to final size, is discarded after
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
 # Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
 # Up to this point, if our dependency tree stays the same,
 # all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin zero2prod

# Runtime stage
# no reference to std stuff and is just its own binary
# this is our final product
FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime!
COPY configuration configuration

# setting the level for application and which config to pull from
ENV APP_ENVIRONMENT production

# this is what to execute
ENTRYPOINT ["./target/release/zero2prod"]