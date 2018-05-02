#!/bin/bash

echo 'deb http://debian.ethz.ch/debian stretch main contrib' >> /etc/apt/sources.list

apt update
apt upgrade -y
apt install -y clang cmake build-essential libxxf86vm-dev libxrandr-dev xorg-dev libglu1-mesa-dev libxrandr2 libglfw3 libgtk-3-dev libgtksourceview-3.0-dev

set -e
cargo build
cargo test
cargo bench

cd luascad; cargo build; cargo test; cargo bench; cd ..
cd tessellation; cargo build; cargo test; cargo bench; cd ..
