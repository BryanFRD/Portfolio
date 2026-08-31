FROM rust:1-slim AS build
WORKDIR /app
COPY api/ ./
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/portfolio-api /usr/local/bin/portfolio-api
ENV PORT=8080
USER nobody
EXPOSE 8080
CMD ["portfolio-api"]
