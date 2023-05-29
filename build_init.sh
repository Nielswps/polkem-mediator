#!/bin/bash

cp ../substrate/target/release/subkey . && \
podman build -t docker.io/nielswps/polkem-mediator-node-init:latest -f DockerfileInitContainer && \
podman push nielswps/polkem-mediator-node-init && \

rm subkey