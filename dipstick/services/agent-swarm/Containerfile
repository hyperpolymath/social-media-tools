# AI Agent Swarm Service - Deno runtime
FROM denoland/deno:latest

WORKDIR /app

# Copy configuration and source
COPY deno.json ./
COPY src/ ./src/

# Cache dependencies
RUN deno cache src/swarm.ts

EXPOSE 3004

CMD ["deno", "run", "--allow-net", "--allow-read", "--allow-env", "src/swarm.ts"]
