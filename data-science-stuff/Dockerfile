FROM rust:latest
COPY config /config
COPY scripts /scripts
RUN chmod +x scripts/*
WORKDIR /scripts
CMD ./start_jupyter.sh
RUN useradd -m rust
RUN apt update && apt install -y \
    jupyter-notebook \
    cmake
RUN cargo install evcxr_jupyter

RUN evcxr_jupyter --install

