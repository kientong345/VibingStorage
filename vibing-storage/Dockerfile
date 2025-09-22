# Build stage
FROM rust:1.79-slim-bullseye AS builder

RUN apt-get update && apt-get install -y build-essential

WORKDIR /usr/src/vibing-storage

# accept the build arguments
ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

RUN cargo build --release


# Production stage
FROM debian:bullseye-slim AS final

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/vibing-storage/target/release/vibing-storage .

# copy runtime configuration files
COPY config.json .
COPY migrations ./migrations

EXPOSE 3001

CMD ["./vibing-storage"]
