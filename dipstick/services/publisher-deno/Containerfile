# Publisher Service - Deno runtime
FROM denoland/deno:latest

WORKDIR /app

# Copy configuration and source
COPY deno.json ./
COPY src/ ./src/

# Cache dependencies
RUN deno cache src/publisher.ts

EXPOSE 3003

CMD ["deno", "run", "--allow-net", "--allow-read", "--allow-env", "src/publisher.ts"]
