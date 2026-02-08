# ReScript GraphQL Gateway - Deno runtime
FROM denoland/deno:latest

WORKDIR /app

# Install ReScript compiler
RUN deno install -A npm:rescript@11.0.0

# Copy configuration
COPY bsconfig.json deno.json ./
COPY src/ ./src/

# Compile ReScript
RUN deno run -A npm:rescript build

# Cache dependencies
RUN deno cache src/Gateway.bs.js

EXPOSE 8000

CMD ["deno", "run", "--allow-net", "--allow-read", "--allow-env", "src/Gateway.bs.js"]
