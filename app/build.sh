#!/usr/bin/env bash

NODE_ENV=production
parcel build --public-url ./ index.html
cp -r dist ../static/app
