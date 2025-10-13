# Earthly CI for vrdx

VERSION 0.8

# Base target for common setup
base:
    FROM alpine:latest
    RUN apk add --no-cache curl git
    RUN curl -LsSf https://astral.sh/uv/install.sh | sh
    ENV PATH="/root/.local/bin:$PATH"
    RUN uv python install 3.13.7
    COPY . .
    RUN uv sync

# Test on Ubuntu
ubuntu:
    FROM ubuntu:22.04
    RUN apt-get update && apt-get install -y curl git ca-certificates
    RUN curl -LsSf https://astral.sh/uv/install.sh | sh
    ENV PATH="/root/.local/bin:$PATH"
    RUN uv python install 3.13.7
    COPY . .
    RUN uv sync
    RUN uv run --with pytest pytest

# Test on Alpine
alpine:
    FROM alpine:latest
    RUN apk add --no-cache curl git ca-certificates
    RUN curl -LsSf https://astral.sh/uv/install.sh | sh
    ENV PATH="/root/.local/bin:$PATH"
    RUN uv python install 3.13.7
    COPY . .
    RUN uv sync
    RUN uv run --with pytest pytest

# Test on CentOS
centos:
    FROM centos:7
    RUN yum install -y curl git ca-certificates
    RUN curl -LsSf https://astral.sh/uv/install.sh | sh
    ENV PATH="/root/.local/bin:$PATH"
    RUN uv python install 3.13.7
    COPY . .
    RUN uv sync
    RUN uv run --with pytest pytest
