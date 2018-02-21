#!/bin/bash

echo 'deb http://debian.ethz.ch/debian stretch main contrib' >> /etc/apt/sources.list

apt update
apt upgrade -y
apt install -y clang cmake build-essential libxxf86vm-dev libxrandr-dev xorg-dev libglu1-mesa-dev libxrandr2 libglfw3 libgtk-3-dev

cargo build
cargo test
cargo bench
cd stl_io; cargo build; cargo test; cd ..
cd luascad; cargo build; cargo test; cargo bench; cd ..
cd primitive; cargo build; cargo test; cargo bench; cd ..
cd tessellation; cargo build; cargo test; cargo bench; cd ..
