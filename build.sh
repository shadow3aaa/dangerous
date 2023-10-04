#!/bin/bash
mkdir -p output
rm -rf output/*
cargo b -r --target aarch64-linux-android
cd output
cp -r ../module/* .
cp -f ../target/aarch64-linux-android/release/dangerous .
strip ./dangerous
zip -9 -rq dangerous.zip .
