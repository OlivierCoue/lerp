FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu
ARG DEBIAN_FRONTEND=noninteractive 
ENV PATH="/root/.cargo/bin:${PATH}"
ARG TARGET_ENV
ENV TARGET_ENV = ${TARGET_ENV}
RUN \
    apt-get update && \
    apt-get -y install --no-install-recommends \
    lld clang cmake llvm \
    python3 python3-pip \
    gcc-mingw-w64 g++-mingw-w64 \
    curl pkg-config libssl-dev 
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
RUN rustup default stable
RUN rustup component add rls rust-analysis rust-src rustfmt clippy rust-std-x86_64-pc-windows-gnu 
RUN pip3 install cargo-lambda

