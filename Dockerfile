FROM rust:slim-buster as builder
WORKDIR /code

ENV SQLX_OFFLINE=1
COPY . .
RUN cargo b --release \
    && strip target/release/tv-calendar

# 
FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /code/target/release/tv-calendar .
ENTRYPOINT [ "./tv-calendar" ]
CMD []
