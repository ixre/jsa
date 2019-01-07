#!/usr/bin/env bash

NODE_ENV=production
rm -rf ./dist && rm -rf ../static/app && \
    parcel build --no-cache --public-url ./ index.html && \
    cp -r dist ../static/app && \
    find ../static/app -name "*.map"|xargs rm -rf
