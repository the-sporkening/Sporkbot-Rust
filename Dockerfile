FROM rust:1.42

WORKDIR /usr/src/sporkbot_rs
COPY . .

RUN cargo install --path .

CMD ["sporkbot_rs"]