FROM amazonlinux:2023

RUN yum update

COPY ./target/release/rust-server-game /usr/app/

CMD [ "/usr/app/rust-server-game" ]
