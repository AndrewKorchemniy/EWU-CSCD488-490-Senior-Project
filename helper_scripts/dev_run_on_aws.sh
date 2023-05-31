#!/bin/sh

# color variables
Color_Off='\033[0m'       # Text Reset
Cyan='\033[0;36m'         # Cyan
Purple='\033[0;35m'       # Purple

cd /home/tztz8/dev/EWU-CSCD488-490-Senior-Project

echo -e "$Cyan Build... $Color_Off"
sh ./helper_scripts/build.sh
echo -e "$Cyan Stop old server... $Color_Off"
ssh ec2-user@aws.tftinker.tech 'sudo systemctl stop actix'
ssh ec2-user@aws.tftinker.tech 'kill $(lsof -ti:8443)'

echo -e "$Cyan Copy new server... $Color_Off"
# backend
scp target/release/backend ec2-user@aws.tftinker.tech:/home/ec2-user/server
scp server.config.toml ec2-user@aws.tftinker.tech:/home/ec2-user/server
scp secret.config.toml ec2-user@aws.tftinker.tech:/home/ec2-user/server
# TODO: add dbcli
# pages
ssh ec2-user@aws.tftinker.tech 'mv /home/ec2-user/server/studentpage /home/ec2-user/server/studentpage-old'
ssh ec2-user@aws.tftinker.tech 'mkdir -p /home/ec2-user/server/studentpage'
scp -r studentpage/dist ec2-user@aws.tftinker.tech:/home/ec2-user/server/studentpage/dist
ssh ec2-user@aws.tftinker.tech 'mv /home/ec2-user/server/adminpage /home/ec2-user/server/adminpage-old'
ssh ec2-user@aws.tftinker.tech 'mkdir -p /home/ec2-user/server/adminpage'
scp -r adminpage/dist ec2-user@aws.tftinker.tech:/home/ec2-user/server/adminpage/dist
# res folder
ssh ec2-user@aws.tftinker.tech 'mv /home/ec2-user/server/res /home/ec2-user/server/res-old'
scp -r adminpage/dist/* ec2-user@aws.tftinker.tech:/home/ec2-user/server/res
scp -r studentpage/dist/* ec2-user@aws.tftinker.tech:/home/ec2-user/server/res
ssh ec2-user@aws.tftinker.tech 'rm /home/ec2-user/server/res/index.html'

#echo -e "$Cyan SSH to start new server... $Color_Off"
#echo -e "$Purple run ' cd /home/ec2-user/server && ./backend && exit '... $Color_Off"
#ssh ec2-user@aws.tftinker.tech
#ssh ec2-user@aws.tftinker.tech '/usr/bin/sh /home/ec2-user/server/run.sh'
echo -e "$Cyan Start server... $Color_Off"
ssh ec2-user@aws.tftinker.tech 'sudo systemctl start actix'
ssh ec2-user@aws.tftinker.tech 'sudo systemctl status actix'
echo -e "$Purple Done... $Color_Off"