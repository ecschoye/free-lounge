FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY flag.txt ./

RUN cargo build --release

EXPOSE 7878

CMD ["cargo", "run", "--release"]
