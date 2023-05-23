#!/bin/sh

# color variables
Color_Off='\033[0m'       # Text Reset
Cyan='\033[0;36m'         # Cyan
Purple='\033[0;35m'       # Purple

cd /home/tztz8/dev/EWU-CSCD488-490-Senior-Project

echo -e "$Cyan Build... $Color_Off"
cd backend
cargo build --release
cd ..
cd adminpage
trunk build --release
cd ..
cd studentpage
trunk build --release
cd ..
mkdir -p res
cp adminpage/dist/* res
cp studentpage/dist/* res
rm res/index.html
echo -e "$Cyan Stop old server... $Color_Off"
sudo systemctl stop actix
kill $(lsof -ti:8443)

echo -e "$Cyan Start new server... $Color_Off"
cargo run

rm -rf res
echo -e "$Purple Done... $Color_Off"