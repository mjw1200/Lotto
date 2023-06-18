FROM rust:1.67

WORKDIR /usr/src/lotto
COPY . .

RUN cargo install --path .

CMD ["lotto"]