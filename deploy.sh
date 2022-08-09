#!/bin/bash

cd iwlist-formatter
~/.cargo/bin/cross build --release --target armv7-unknown-linux-musleabihf

servername="root@10.42.0.28"
passwd="root"

sshpass -p $passwd ssh $servername "bash -c \"ifsctl mnt rootfs rw\""

sshpass -p $passwd ssh $servername "bash -c \"rm /iwlist-formatter\""

sshpass -p $passwd scp target/armv7-unknown-linux-musleabihf/release/iwlist-formatter $servername:/

sshpass -p $passwd ssh $servername "bash -c \"sync\""

# Normal launch
sshpass -p $passwd ssh $servername "bash -c \"/iwlist-formatter\""

# For chroot
#sshpass -p $passwd ssh $servername "bash -c \"rm /kobo/inkbox-power-deamon\""
#sshpass -p $passwd ssh $servername "bash -c \"mv /inkbox-power-deamon /kobo/\""
#shpass -p $passwd ssh $servername "chroot /kobo sh -c \"DEBUG=true /inkbox-power-deamon\""

# Reconnect to wifi
# /usr/local/bin/wifi/connect_to_network.sh hotspot 12345678
