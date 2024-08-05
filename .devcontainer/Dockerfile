FROM mcr.microsoft.com/devcontainers/base:debian-12
#RUN userdel ubuntu && usermod -u 1000 vscode

ARG NODE_VERSION=20.16.0
ARG RUST_VERSION=1.80.0

# Add extra APT repositories
#COPY apt/ /etc/apt/

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld pkg-config libudev-dev \
   && apt-get autoremove -y && apt-get clean -y

USER vscode

#ENV HOME=/home/ubuntu
#ENV PATH=${PATH}:${HOME}/.node/bin:${HOME}/.cargo/bin:${HOME}/.local/bin

#RUN mkdir ${HOME}/.node && \
#    curl -fsSL https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz | \
#    tar -xJ -C ${HOME}/.node --strip-components 1

#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain $RUST_VERSION --no-modify-path -y

#RUN cargo install webwire-cli tokio-console
#RUN cargo install --locked cargo-outdated
#RUN cargo install cross
