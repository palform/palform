FROM docker.io/rust:1-slim-bookworm as builder

RUN apt-get update
RUN apt-get install -y libssl-dev pkg-config

WORKDIR /usr/src/palform
COPY packages/backend packages/backend
COPY packages/analysis packages/analysis
COPY packages/bench packages/bench
COPY packages/client-common packages/client-common
COPY packages/entities packages/entities
COPY packages/migration packages/migration
COPY packages/google-fonts-generator packages/google-fonts-generator
COPY packages/tsid packages/tsid
COPY Cargo.toml Cargo.lock Rocket.toml ./

RUN cargo install --path packages/backend --profile release

FROM docker.io/debian:bookworm-slim
RUN apt-get update
RUN apt-get install -y libssl-dev pkg-config ca-certificates
RUN update-ca-certificates

COPY --from=builder /usr/local/cargo/bin/palform-backend /opt/palform-backend

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV ROCKET_LOG_LEVEL=normal

CMD ["/opt/palform-backend"]
