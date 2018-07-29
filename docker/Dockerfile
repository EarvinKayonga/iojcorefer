FROM    rust:1.27.2-slim-stretch as builder

ADD     . /app

WORKDIR /app

RUN     cargo build --release

FROM    debian:stable-slim as release
COPY --from=builder /app/target/release/iojcorefer /app/server
CMD     /app/server
