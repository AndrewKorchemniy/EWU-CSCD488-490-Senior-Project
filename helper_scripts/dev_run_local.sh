#!/bin/sh

# color variables
Color_Off='\033[0m'       # Text Reset
Cyan='\033[0;36m'         # Cyan
Purple='\033[0;35m'       # Purple

cd /mnt/c/Users/theha/Desktop/true\ senior\ project/EWU-CSCD488-490-Senior-Project

echo -e "$Cyan Build... $Color_Off"
sh ./helper_scripts/build.sh
echo -e "$Cyan Stop old server... $Color_Off"
#sudo systemctl stop actix
kill $(lsof -ti:8443)

echo -e "$Cyan Start new server... $Color_Off"
cargo run

rm -rf res
echo -e "$Purple Done... $Color_Off"