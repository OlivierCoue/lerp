FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu
ARG DEBIAN_FRONTEND=noninteractive 
ENV PATH="/root/.cargo/bin:${PATH}"
ARG TARGET_ENV
ARG SERVER_GAME_IP
ARG AWS_ENDPOINT_URL
ENV TARGET_ENV = ${TARGET_ENV}
ENV SERVER_GAME_IP = ${SERVER_GAME_IP}
ENV SERVER_GAME_IP = ${AWS_ENDPOINT_URL}
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

# AWS CLI
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
RUN unzip awscliv2.zip
RUN ./aws/install

# Node
RUN curl -sL https://deb.nodesource.com/setup_22.x | bash
RUN apt-get install nodejs

RUN apt-get install -y apt-transport-https ca-certificates curl gnupg2 lsb-release \
    && curl -fsSL https://download.docker.com/linux/$(lsb_release -is | tr '[:upper:]' '[:lower:]')/gpg | apt-key add - 2>/dev/null \
    && echo "deb [arch=amd64] https://download.docker.com/linux/$(lsb_release -is | tr '[:upper:]' '[:lower:]') $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list \
    && apt-get update \
    && apt-get install -y docker-ce-cli

# Install Docker Compose
RUN LATEST_COMPOSE_VERSION=$(curl -sSL "https://api.github.com/repos/docker/compose/releases/latest" | grep -o -P '(?<="tag_name": ").+(?=")') \
    && curl -sSL "https://github.com/docker/compose/releases/download/${LATEST_COMPOSE_VERSION}/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose \
    && chmod +x /usr/local/bin/docker-compose

# For Bevy and some dependancy
RUN apt-get -y install --no-install-recommends \
    libasound2-dev \
    libudev-dev \
    libwayland-dev