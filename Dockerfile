# jarry's short address service images
# Version : 0.1
# Author : jarrysix(jarrysix@gmail.com)
# Date : 2018-10-21 22:39

FROM ekidd/rust-musl-builder AS builder
# Add our source code.
ADD . ./
# Fix permissions on source code and Build our application.
RUN sudo chown -R rust:rust ../ && rustup default nightly &&\
    cargo build --release

FROM alpine:latest
MAINTAINER jarrysix
LABEL Vendor="github.com/jsix"
LABEL License="GPLv2"
LABEL Version=1.0.0

WORKDIR /usr/bin
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/jsa \
    /usr/bin/jsa

VOLUME ["/jsa"]
EXPOSE 8302
ENTRYPOINT jsa -c /jsa



