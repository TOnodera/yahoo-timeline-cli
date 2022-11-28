FROM rust:latest
RUN apt-get update -y && apt-get install -y vim
RUN useradd -m -u 1000 -s /bin/bash rust 
RUN rustup component add rustfmt
USER rust