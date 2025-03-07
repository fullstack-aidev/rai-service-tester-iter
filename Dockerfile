# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Update rust and cargo to the latest stable version
RUN rustup update stable

# Copy only the necessary files and directories for the build into the container at /usr/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY config_iter.yml ./

# Build the Rust application
RUN cargo build --release

FROM debian:stable-slim

# Install required libraries
RUN apt-get update && apt-get install -y \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir /usr/app /usr/app/response_payload

WORKDIR /usr/app

COPY --from=builder /usr/app/target/release/rai-service-tester-iter .
COPY --from=builder /usr/app/config_iter.yml .

# Expose the port that the application will run on
EXPOSE 3030

# Run the application in servertest mode
CMD ["./rai-service-tester-iter"]