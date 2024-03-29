FROM rust:latest AS builder

COPY ./ ./

ENV PORT=3000

RUN cargo build --release

FROM debian:stable-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /target/release/WssDedicated /target/release/WssDedicated

EXPOSE 3000

CMD ["/target/release/WssDedicated"]
