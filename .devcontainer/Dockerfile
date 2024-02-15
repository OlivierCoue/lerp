FROM mcr.microsoft.com/vscode/devcontainers/rust:1-bullseye

RUN \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends \
    lld clang cmake \
    iftop \ 
    docker \
    python3 \
    python3-pip\
    && \
    rustup toolchain install nightly && \
    rustup default && \
    rustup component add rls rust-analysis rust-src rustfmt clippy rust-std-x86_64-pc-windows-gnu \
    && \
    pip3 install cargo-lambda