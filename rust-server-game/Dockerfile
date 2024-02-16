FROM ubuntu:20.04

RUN apt-get update

WORKDIR /usr/app

COPY ./target/release/rust-server-game /usr/app/

CMD [ "rust-server-game" ]