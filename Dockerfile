# jarry's short address service images
# Version : 0.1
# Author : jarrysix(jarrysix@gmail.com)
# Date : 2018-10-21 22:39

FROM ekidd/rust-musl-builder:latest AS builder
# Add our source code.
ADD ./Cargo.toml ./
ADD ./src ./src
# Fix permissions on source code and Build our application.
RUN sudo chown -R rust:rust ../ && \
 rustup default nightly && rustup target add x86_64-unknown-linux-musl &&\
 rustup update &&\
 cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
MAINTAINER jarrysix
LABEL Vendor="github.com/jsix"
LABEL License="GPLv2"
LABEL Version=1.0.0

WORKDIR /jsa
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/jsa .
COPY ./migrations ./migrations
COPY ./static ./static
COPY ./app ./app

VOLUME ["/data"]
EXPOSE 8302
ENTRYPOINT ./jsa -d /data



