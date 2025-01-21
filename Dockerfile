# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

# Set the command to run the application
CMD ["./target/release/rai-service-tester-iter"]