# Multi-stage build for ReScript analyzer with WASM optimization
FROM rust:1.75-slim as wasm-builder

WORKDIR /build/wasm
COPY wasm/ .

# Install wasm-pack for WASM builds
RUN cargo install wasm-pack

# Build WASM module with optimizations
RUN cargo build --release --target wasm32-unknown-unknown

# ReScript + Deno stage
FROM denoland/deno:latest

WORKDIR /app

# Install ReScript compiler
RUN deno install -A npm:rescript@11.0.0

# Copy source files
COPY bsconfig.json deno.json ./
COPY src/ ./src/

# Copy WASM artifacts from builder
COPY --from=wasm-builder /build/wasm/target/wasm32-unknown-unknown/release/*.wasm ./wasm/

# Compile ReScript to JavaScript
RUN deno run -A npm:rescript build

# Cache Deno dependencies
RUN deno cache src/Main.bs.js

EXPOSE 3002

CMD ["deno", "run", "--allow-net", "--allow-read", "--allow-env", "src/Main.bs.js"]
