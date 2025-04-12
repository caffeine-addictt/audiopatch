FROM rust:1.85.1 AS builder
WORKDIR /app

# musl
RUN apt-get update \
  && apt-get install --no-install-recommends -y musl-tools \
  && rm -rf /var/lib/apt/lists/* \
  && rustup target add x86_64-unknown-linux-musl

# caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
  && cargo fetch && rm -r src

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM alpine:3.21.3
ARG APP_DIR=/usr/src/app
ARG APP_USER=appuser
LABEL author=caffeine-addictt
LABEL contact=contact@ngjx.org

WORKDIR ${APP_DIR}

RUN apk update \
  && apk add --no-cache ca-certificates tzdata \
  && rm -rf /var/cache/apk/*

RUN addgroup -S ${APP_USER} && adduser -S -g ${APP_USER} ${APP_USER} \
  && chown -R ${APP_USER}:${APP_USER} ${APP_DIR}

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/audiopatch /usr/bin/audiopatch

USER $APP_USER

ENV RUST_LOG="info"
ENV ENVIRONMENT="production"
EXPOSE 3000
ENTRYPOINT [ "/usr/bin/audiopatch" ]
