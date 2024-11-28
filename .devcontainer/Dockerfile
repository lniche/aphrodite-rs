FROM rust:1.80

WORKDIR /workspace

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    git \
    curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt && \
    rustup component add clippy && \
    rustup component add rust-src

RUN rustup toolchain install nightly && \
    rustup component add rust-analyzer --toolchain nightly

RUN rustup component add llvm-tools-preview && \
    cargo install cargo-watch && \
    cargo install cargo-edit

COPY . .

EXPOSE 8000

CMD ["/bin/bash"]
