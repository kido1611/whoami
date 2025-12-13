# ================================================================================= NODE Section
FROM node:25.2.1-alpine AS node-base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN npm install -g pnpm@latest

WORKDIR /app
COPY package.json /app/
COPY pnpm-lock.yaml /app/

FROM node-base AS node-deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

FROM node-base AS node-build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
COPY ./assets /app/assets
COPY ./templates /app/templates
RUN pnpm run build

# ================================================================================= RUST Section
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY --from=node-build /app/dist/style.css /app/dist/style.css
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static-debian13:nonroot AS runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/whoiam /app/
USER nonroot

ENV RUST_LOG=info \
  WHOIAM_IP_SOURCE=ConnectInfo \
  WHOIAM_PORT=8080

EXPOSE 8080

ENTRYPOINT ["/app/whoiam"]
