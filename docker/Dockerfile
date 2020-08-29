FROM    rust:1.46-slim-buster as builder

ADD     . /app

WORKDIR /app

RUN     cargo build --release

FROM    debian:stable-slim as release
COPY --from=builder /app/target/release/iojcorefer /app/server
COPY --from=builder /app/static /static
CMD     /app/server
