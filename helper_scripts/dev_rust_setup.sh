#!/bin/sh

Color_Off='\033[0m'       # Text Reset
Cyan='\033[0;36m'         # Cyan
Purple='\033[0;35m'       # Purple

# get packages
echo -e "$Cyan Updating System... $Color_Off"
sudo dnf update
sudo dnf upgrade -y
echo -e "$Cyan Install Development Tools, openssl-devel, mariadb-connector-c ... $Color_Off"
sudo dnf groupinstall 'Development Tools'
sudo dnf install openssl-devel mariadb-connector-c
# get rustup
echo -e "$Cyan Install rustup ... $Color_Off"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# TODO: run src
echo -e "$Cyan Install clippy, wasm, trunk ... $Color_Off"
rustup component add clippy
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
echo -e "$Cyan TODO: Install db ... $Color_Off"
# TODO: install db