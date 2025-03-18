# Use Rust as the base image and install Node.js manually
FROM rust:1.85 AS builder

# Install required dependencies (Clang, Node.js, npm)
RUN apt-get update && apt-get install -y curl pkg-config build-essential clang \
    && curl -fsSL https://deb.nodesource.com/setup_14.x | bash - \
    && apt-get install -y nodejs npm

# Verify installations
RUN rustc --version && cargo --version && node -v && npm -v

# Set working directory
WORKDIR /usr/src/app

# Install a specific version of wasm-pack (v0.10.1 from 2021)
RUN curl -L -o wasm-pack.tar.gz https://github.com/rustwasm/wasm-pack/releases/download/v0.10.1/wasm-pack-v0.10.1-x86_64-unknown-linux-musl.tar.gz \
    && tar -xzf wasm-pack.tar.gz \
    && mv wasm-pack-v0.10.1-x86_64-unknown-linux-musl/wasm-pack /usr/local/bin/wasm-pack \
    && chmod +x /usr/local/bin/wasm-pack \
    && rm -rf wasm-pack.tar.gz wasm-pack-v0.10.1-x86_64-unknown-linux-musl

# Copy project files
COPY . .

# Build Rust WASM package
# WORKDIR /usr/src/app/rust
# RUN cargo build
# # Install Rust WASM target before running wasm-pack
# RUN rustup target add wasm32-unknown-unknown
# RUN wasm-pack build --target=nodejs

# # Install Node.js dependencies
# WORKDIR /usr/src/app
# RUN npm install
# RUN npm run rust:build-nodejs
