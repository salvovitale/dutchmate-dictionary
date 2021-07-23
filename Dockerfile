FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release


FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/app /usr/local/bin/app
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["app"]