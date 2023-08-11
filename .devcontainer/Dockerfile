# ベースとなるRustのイメージを選択
FROM rust:1.71.1-bullseye

# Rust関連のセットアップ
RUN apt update && \
    apt -y install git sudo make wget libgl1-mesa-glx libegl1-mesa&& \
    apt clean && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch

# MiniZincのアーカイブ版をダウンロード、展開
WORKDIR /tmp
RUN wget https://github.com/MiniZinc/MiniZincIDE/releases/download/2.7.6/MiniZincIDE-2.7.6-bundle-linux-x86_64.tgz -O MiniZincIDE-2.7.6-bundle-linux-x86_64.tgz
RUN tar xf MiniZincIDE-2.7.6-bundle-linux-x86_64.tgz

# minizincのラッパースクリプトを作成
RUN echo '#!/bin/bash' > /usr/local/bin/minizinc-wrapper && \
    echo 'export LD_LIBRARY_PATH="/tmp/MiniZincIDE-2.7.6-bundle-linux-x86_64/lib:${LD_LIBRARY_PATH}"' >> /usr/local/bin/minizinc-wrapper && \
    echo 'export PATH="/tmp/MiniZincIDE-2.7.6-bundle-linux-x86_64/bin:${PATH}"' >> /usr/local/bin/minizinc-wrapper && \
    echo 'minizinc "$@"' >> /usr/local/bin/minizinc-wrapper && \
    chmod +x /usr/local/bin/minizinc-wrapper

ENV SHELL bash
ARG USERNAME=node
RUN useradd -m $USERNAME && \
    echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME && \
    chmod 0440 /etc/sudoers.d/$USERNAME

# RustとMiniZincのエイリアスを設定
RUN echo "alias c='cargo'" >> /home/node/.bashrc && \
    echo "alias cr='cargo run --release'" >> /home/node/.bashrc && \
    echo "alias crd='cargo run'" >> /home/node/.bashrc && \
    echo "alias cc='cargo check'" >> /home/node/.bashrc && \
    echo "alias cy='cargo clippy'" >> /home/node/.bashrc && \
    echo "alias m='minizinc-wrapper --solver gecode --all-solutions'" >> /home/node/.bashrc

USER $USERNAME
WORKDIR /home/node/works
COPY . /home/node/works
