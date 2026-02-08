# PESTLE Observatory Service - Deno runtime
FROM denoland/deno:latest

WORKDIR /app

# Copy configuration and source
COPY deno.json ./
COPY src/ ./src/

# Cache dependencies
RUN deno cache src/observatory.ts

EXPOSE 3005

CMD ["deno", "run", "--allow-net", "--allow-read", "--allow-env", "src/observatory.ts"]
