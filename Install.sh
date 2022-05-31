#!/bin/bash

cp battery_notifier.service battery_notifier_complete.service

echo "[Install.sh] Percentage when the notification should be sent:"
read percentage
sed -i -e "s/PERCENTAGE/$percentage/g" battery_notifier_complete.service

echo "[Install.sh] Build binary ..."
cargo build --release
echo "[Install.sh] Copy files to final locations ..."
sudo cp target/release/battery_notifier /usr/bin/
sudo cp battery_notifier_complete.service /etc/systemd/system/battery_notifier.service
echo "[Install.sh] Enable and start daemon ..."
sudo systemctl enable --now battery_notifier.service
