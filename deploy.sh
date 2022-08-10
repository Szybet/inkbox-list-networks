#!/bin/bash

cd inkbox-list-networks
~/.cargo/bin/cross +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release --target armv7-unknown-linux-musleabihf

upx --best --lzma target/armv7-unknown-linux-musleabihf/release/inkbox-list-networks

servername="root@10.42.0.28"
passwd="root"

sshpass -p $passwd ssh $servername "bash -c \"ifsctl mnt rootfs rw\""

sshpass -p $passwd ssh $servername "bash -c \"rm /inkbox-list-networks\""

sshpass -p $passwd scp target/armv7-unknown-linux-musleabihf/release/inkbox-list-networks $servername:/

sshpass -p $passwd ssh $servername "bash -c \"sync\""

# Normal launch
sshpass -p $passwd ssh $servername "bash -c \"/inkbox-list-networks\""

# For chroot
#sshpass -p $passwd ssh $servername "bash -c \"rm /kobo/inkbox-power-deamon\""
#sshpass -p $passwd ssh $servername "bash -c \"mv /inkbox-power-deamon /kobo/\""
#shpass -p $passwd ssh $servername "chroot /kobo sh -c \"DEBUG=true /inkbox-power-deamon\""

# Reconnect to wifi
# /usr/local/bin/wifi/connect_to_network.sh hotspot 12345678
