#!/bin/sh

Color_Off='\033[0m'       # Text Reset
Cyan='\033[0;36m'         # Cyan
Purple='\033[0;35m'       # Purple

# get packages
echo -e "$Cyan Updating System... $Color_Off"
sudo dnf update
sudo dnf upgrade -y
echo -e "$Cyan Install Development Tools, openssl/dev, mariadb-connector-c/dev ... $Color_Off"
sudo dnf groupinstall 'Development Tools'
# mariadb-connector-c gives the mysql client lib
sudo dnf install openssl openssl-devel mariadb-connector-c mariadb-connector-c-devel
# get rustup
echo -e "$Cyan Install rustup ... $Color_Off"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
echo -e "$Cyan Install clippy, wasm ... $Color_Off"
rustup component add clippy
rustup target add wasm32-unknown-unknown
echo -e "$Cyan Install trunk ... $Color_Off"
cargo install --locked trunk
echo -e "$Cyan Install db ... $Color_Off"
cargo install diesel_cli --no-default-features --features mysql
echo -e "$Cyan Getting project ... $Color_Off"
git clone https://github.com/tztz8/EWU-CSCD488-490-Senior-Project.git