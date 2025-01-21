# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/rai-service-tester

# Copy only the necessary files and directories for the build into the container at /usr/src/rai-service-tester
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY config_iter.yml ./


# Build the Rust application
RUN cargo build --release

# Expose the port that the application will run on
EXPOSE 3030

# Run the application in servertest mode
CMD ["./target/release/rai-service-tester-iter"]