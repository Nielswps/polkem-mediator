#!/bin/bash

cargo build --release && \
cp target/release/polkem-mediator-node . && \
podman build -t docker.io/nielswps/polkem-mediator-node:latest . && \
podman push nielswps/polkem-mediator-node && \
rm polkem-mediator-node
