#!/usr/bin/env bash

jail=/cellblock/piptest
mkdir $jail

# The big three.
mkdir $jail/proc $jail/sys  $jail/dev
mount --rbind /dev $jail/dev/
mount --rbind /sys $jail/sys/
mount -t proc /proc $jail/proc/

# This is somewhat violative of the principle of least privilege.
# The principle of least privilege is violative of my time, however.
# Anyway, I should have nothing to worry about a standard /usr/lib.
./jailer.sh /usr/lib  $jail

# /etc, however can get dicey. ssl keys, IP addys. We'll keep this minimal.
./jailer.sh /etc/ssl/certs/ $jail

# I could get flagrant with /usr/include if I want, but this seems to work.
./jailer.sh /usr/include/python3.10/ $jail

# Big dog.
./jailer.sh /usr/bin/pip3 $jail
./jailer.sh /usr/bin/env $jail

# For testing purposes.
./jailer.sh /usr/bin/curl  $jail

# It... was DNS.
./jailer.sh /etc/host.conf  $jail
cp /run/systemd/resolve/stub-resolv.conf $jail/etc/resolv.conf


