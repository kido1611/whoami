# ================================================================================= NODE Section
FROM node:25.2.1-alpine AS node-base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN npm install -g pnpm@latest

WORKDIR /app
COPY package.json /app/
COPY pnpm-lock.yaml /app/

FROM node-base AS node-build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
COPY ./assets /app/assets
COPY ./templates /app/templates
RUN pnpm run build

# ================================================================================= RUST Section
FROM clux/muslrust:stable AS chef
ARG TARGETARCH
RUN case "$TARGETARCH" in \
  amd64)  echo "x86_64-unknown-linux-musl" > /tmp/target ;; \
  arm64)  echo "aarch64-unknown-linux-musl" > /tmp/target ;; \
  *) echo "unsupported arch: $TARGETARCH" && exit 1 ;; \
esac
RUN rustup target add "$(cat /tmp/target)"
USER root
RUN cargo install cargo-chef
WORKDIR /app

# ================================================================================= CHEF PREPARE
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ================================================================================= BUILD APP
FROM chef AS builder
ARG TARGETARCH
ARG BUILT_GIT_COMMIT_HASH
ENV BUILT_GIT_COMMIT_HASH=$BUILT_GIT_COMMIT_HASH

COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,id=cargo-registry-${TARGETARCH},target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git-${TARGETARCH},target=/usr/local/cargo/git \
    cargo chef cook --release --target "$(cat /tmp/target)" --recipe-path recipe.json

COPY --from=node-build /app/dist/style.css /app/dist/style.css
COPY . .

RUN --mount=type=cache,id=cargo-registry-${TARGETARCH},target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git-${TARGETARCH},target=/usr/local/cargo/git \
    cargo build --release --target "$(cat /tmp/target)" && \
    cp "/app/target/$(cat /tmp/target)/release/whoami" "/app/whoami"

# ================================================================================= RUNTIME
FROM gcr.io/distroless/static-debian13:nonroot AS runtime

LABEL org.opencontainers.image.title=whoami \
  org.opencontainers.image.description="Alternative ifconfig.me, Detect your IP address" \
  org.opencontainers.image.url=https://github.com/kido1611/whoami \
  org.opencontainers.image.source=https://github.com/kido1611/whoami \
  org.opencontainers.image.vendor="Muhammad Abdusy Syukur"

WORKDIR /app
COPY --from=builder /app/whoami /app/
USER nonroot

ENV RUST_LOG=info \
  WHOAMI_IP_SOURCE=ConnectInfo \
  WHOAMI_PORT=8080

EXPOSE 8080

ENTRYPOINT ["/app/whoami"]
