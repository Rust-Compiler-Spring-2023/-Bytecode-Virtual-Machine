FROM rust:latest
WORKDIR /app/src
COPY . /app/
RUN apt-get update && apt-get install -y openssl
RUN cargo build --release
CMD ["cargo", "run", "../test/welcome.lox"]
