FROM amazonlinux:2023

RUN dnf update -y \
    && dnf install -y git tar gzip nodejs20 npm openssl-devel \
    && dnf groupinstall -y "Development Tools" \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY .bashrc /root/.bashrc
