# Stage 1: Build the Rust application
FROM rust:1.88-slim-bullseye AS builder

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Set working directory to the api directory
WORKDIR /usr/src/api

# Copy the entire repository
COPY . .

# Ensure we're building from the api directory
WORKDIR /usr/src/api

# Build the release version
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/api/target/release/nekoi-api /usr/local/bin/nekoi-api

# Run the application
CMD ["/usr/local/bin/nekoi-api"]
