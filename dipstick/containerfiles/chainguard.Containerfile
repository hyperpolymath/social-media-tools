# syntax=docker/dockerfile:1.4
ARG LOCAL_REPOS=/var/mnt/eclipse/repos

##### Stage: Collector (Rust) #####
FROM cgr.dev/chainguard/wolfi-base:3.20 as collector-builder
RUN apk add --no-cache bash build-base curl git openssl-dev pkgconf
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /collector
COPY services/collector/ .
RUN cargo build --release

##### Stage: Analyzer + Publisher (ReScript/Deno) #####
FROM denoland/deno:2 as deno-builder
WORKDIR /workspace
COPY services/analyzer-rescript/ analyzer/
COPY services/gateway-rescript/ gateway/
COPY services/agent-swarm/ agent-swarm/
COPY services/publisher-deno/ publisher/
RUN cd analyzer && deno task build
RUN cd gateway && deno task build
RUN cd publisher && deno task build

##### Stage: Dashboard (Elixir) #####
FROM docker.io/library/elixir:1.15 as dashboard-builder
WORKDIR /dashboard
COPY services/dashboard/ .
RUN mix deps.get && MIX_ENV=prod mix release

##### Stage: Scraper (Julia) #####
FROM julialang/julia:1.10 as julia-builder
WORKDIR /scraper
COPY services/scraper-julia/ .
RUN julia --project=. -e 'using Pkg; Pkg.instantiate(); Pkg.precompile();'

##### Stage: Ada TUI #####
FROM cgr.dev/chainguard/wolfi-base:3.20 as ada-builder
RUN apk add --no-cache gnat gnatls
WORKDIR /tui
COPY services/tui-ada/ .
RUN gnatmake src/nuj_tui.adb -b

##### Stage: Infra Assets #####
FROM cgr.dev/chainguard/wolfi-base:3.20 as infra-builder
WORKDIR /infra
COPY services/xtdb-temporal/ /infra/xtdb/
COPY infrastructure/database/ /infra/database/
COPY services/pestle-observatory/ /infra/pestle/

##### Stage: Selur/Svalinn/Vörðr Tooling #####
FROM cgr.dev/chainguard/wolfi-base:3.20 as admin-cli-builder
ARG LOCAL_REPOS=/var/mnt/eclipse/repos
RUN apk add --no-cache bash curl git build-base pkgconf openssl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN curl -fsSL https://deno.land/install.sh | sh -s -- -n deno
ENV PATH="/root/.cargo/bin:/root/.deno/bin:/usr/local/bin:${PATH}"
WORKDIR /admin
RUN mkdir -p /admin/bin

RUN --mount=type=bind,source=${LOCAL_REPOS}/selur-compose,target=/workspace/selur-compose \
    cd /workspace/selur-compose && cargo build --release && cp target/release/selur-compose /admin/bin/

RUN --mount=type=bind,source=${LOCAL_REPOS}/vordr,target=/workspace/vordr \
    cd /workspace/vordr && cargo build --release && cp target/release/vordr /admin/bin/

RUN --mount=type=bind,source=${LOCAL_REPOS}/cerro-torre,target=/workspace/cerro-torre \
    cd /workspace/cerro-torre && cargo build --release && cp target/release/cerro-torre /admin/bin/

RUN --mount=type=bind,source=${LOCAL_REPOS}/svalinn,target=/workspace/svalinn \
    /root/.deno/bin/deno cache /workspace/svalinn/src/main.ts && cp -r /workspace/svalinn /admin/svalinn

##### Final Stage #####
FROM cgr.dev/chainguard/wolfi-base:3.20
RUN apk add --no-cache bash libgcc libstdc++ openssl ca-certificates
LABEL org.opencontainers.image.title="Social Media Ethics Monitor"
LABEL org.opencontainers.image.licenses="MPL-2.0-or-later"
WORKDIR /app

# Services
COPY --from=collector-builder /collector/target/release/collector /usr/local/bin/collector
COPY --from=deno-builder /workspace/analyzer/dist /app/analyzer
COPY --from=deno-builder /workspace/gateway/dist /app/gateway
COPY --from=deno-builder /workspace/publisher/dist /app/publisher
COPY --from=dashboard-builder /dashboard/_build/prod/rel/social_media_monitor /app/dashboard
COPY --from=julia-builder /scraper /app/scraper
COPY --from=ada-builder /tui/bin/tui /usr/local/bin/tui

# Infra
COPY --from=infra-builder /infra /app/infra

# Orchestration
COPY --from=admin-cli-builder /admin/bin/selur-compose /usr/local/bin/selur-compose
COPY --from=admin-cli-builder /admin/bin/vordr /usr/local/bin/vordr
COPY --from=admin-cli-builder /admin/bin/cerro-torre /usr/local/bin/cerro-torre
COPY --from=admin-cli-builder /admin/svalinn /app/svalinn

COPY config/ /app/config/
ENV MODE=production
ENTRYPOINT ["/usr/local/bin/selur-compose", "up"]
