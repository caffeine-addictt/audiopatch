FROM clux/muslrust:stable AS builder
WORKDIR /app

# caching
RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
  && cargo build --release --target x86_64-unknown-linux-musl && rm -rf src

COPY . .
RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl


FROM alpine:3.22.0
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
