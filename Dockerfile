FROM docker.io/library/rust:1-alpine as build
RUN apk add --no-cache musl-dev
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install --version ^0.6 sqlx-cli --no-default-features --features rustls,sqlite

WORKDIR /build
COPY . .
RUN DATABASE_URL=sqlite:db.sqlite?mode=rwc sqlx migrate run
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    DATABASE_URL=sqlite:db.sqlite \
    cargo build --release && cp target/release/ical-manager ical-manager

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-static /tini
RUN chmod +x /tini

FROM gcr.io/distroless/static

WORKDIR /usr/share/ical-manager
COPY assets assets
COPY .dockerenv .env
COPY --from=build /build/ical-manager /usr/bin/

COPY --from=build /tini /tini
ENTRYPOINT ["/tini", "--"]

ENV DATABASE_URL=sqlite:db.sqlite?mode=rwc
CMD ["ical-manager"]
