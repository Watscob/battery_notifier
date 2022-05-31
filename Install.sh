#!/bin/bash

PATH=/usr/bin

echo "[Install.sh] Build binary ..."
cargo build --release
echo "[Install.sh] Copy files in $PATH"
sudo cp target/release/battery_notifier $PATH
