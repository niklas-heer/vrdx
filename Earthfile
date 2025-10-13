# Earthly CI for vrdx

VERSION 0.8



# Test on Ubuntu
ubuntu:
    FROM ubuntu:24.04
    RUN apt-get update && apt-get install -y curl git ca-certificates
    RUN curl -LsSf https://astral.sh/uv/install.sh | sh
    ENV PATH="/root/.local/bin:$PATH"
    RUN uv python install 3.13
    COPY . .
    RUN uv sync
    RUN uv run --with pytest pytest

# Test on Alpine
alpine:
    FROM alpine:latest
    RUN apk add --no-cache curl git ca-certificates
    RUN curl -LsSf https://astral.sh/uv/install.sh | sh
    ENV PATH="/root/.local/bin:$PATH"
    RUN uv python install 3.13
    COPY . .
    RUN uv sync
    RUN uv run --with pytest pytest

# Test on Fedora
fedora:
    FROM fedora:40
    RUN dnf install -y curl git ca-certificates && dnf clean all
    RUN curl -LsSf https://astral.sh/uv/install.sh | sh
    ENV PATH="/root/.local/bin:$PATH"
    RUN uv python install 3.13
    COPY . .
    RUN uv sync
    RUN uv run --with pytest pytest

# Meta target to run all distro tests
ci:
    FROM scratch
    BUILD +ubuntu
    BUILD +alpine
    BUILD +fedora
