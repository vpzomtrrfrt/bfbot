FROM alpine:3.19
RUN apk add --no-cache rust cargo openssl-dev
WORKDIR /usr/src/bfbot
COPY Cargo.* ./
COPY src ./src
RUN cargo build --release
RUN cp ./target/release/bfbot /usr/bin/

RUN adduser -S bfbot
USER bfbot
CMD ["bfbot"]
