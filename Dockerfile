# Stage 1: Build the application
FROM rust:latest as builder

# Install necessary dependencies for musl target
RUN apt-get update && apt-get -y install \
    ca-certificates cmake musl-tools libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo files first to optimize Docker cache
COPY Cargo.toml Cargo.lock ./
# Ensure source structure is correct
COPY src src

# Create a dummy src directory and run a build to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN rustup default stable && rustup update
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

# Copy the rest of the source files and build the application
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

# Stage 2: Create a minimal final image
FROM scratch

# Copy the compiled binary from the build stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/offchain_server /

# Ensure the templates directory exists in the correct location
COPY --from=builder /app/templates /templates

# Copy necessary certificates (optional, depends on your app's requirements)
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

# Expose the port your app runs on
EXPOSE 8080

# Define the default command to run the application
CMD ["/offchain_server"]

