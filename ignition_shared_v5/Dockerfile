# FROM rust:latest AS builder

# # 2. Copy the files in your machine to the Docker image
# COPY ./ ./

# # Build your program for release
# RUN cargo build --release

# FROM debian:stable-slim
# COPY --from=builder ./target/release/WebsocketServer ./target/release/WebsocketServer
# CMD ["./target/release/WebsocketServer"]

# EXPOSE 3000

# Use Rust official image to build your application
FROM rust:latest AS builder

# Copy the files from your host to your current location.
COPY ./ ./

ENV PORT=3000

# Build your program for release
RUN cargo build --release

# Use Debian stable slim as the runtime container
FROM debian:stable-slim

# Install ca-certificates
# Update the package lists, install ca-certificates, and clean up cache to reduce image size
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage to the runtime container
COPY --from=builder /target/release/ignition_shared_v5 /target/release/ignition_shared_v5
# /target/release/WebsocketServer /target/release/WebsocketServer

# Expose the port the server listens on
EXPOSE 3000

# Command to run the executable
CMD ["/target/release/ignition_shared_v5"]
