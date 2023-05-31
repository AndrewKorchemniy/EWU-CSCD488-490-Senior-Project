# Helper Scripts

## build.sh

build all the parts

## package.sh

make a tar the files need for server

## install.sh

Install

### TODO: Need

- [ ] Install libs (`openssl`)
- [ ] Setup server user (`status-reports`)
- [ ] Copy server into user (`/home/status-reports/server`)
- [ ] Copy SSL cert into user (`/home/status-reports/server`)
```shell
SSL_CERTIFICATE="/etc/letsencrypt/live/aws.tftinker.tech/fullchain.pem"
SSL_KEY="/etc/letsencrypt/live/aws.tftinker.tech/privkey.pem"

sudo cp $SSL_CERTIFICATE /home/status-reports/server/fullchain.pem
sudo chown status-reports:status-reports /home/status-reports/server/fullchain.pem
sudo su - status-reports -c "chmod 400 /home/status-reports/server/fullchain.pem"

sudo cp $SSL_KEY /home/status-reports/server/privkey.pem
sudo chown status-reports:status-reports /home/status-reports/server/privkey.pem
sudo su - status-reports -c "chmod 400 /home/status-reports/server/privkey.pem"
```
- [ ] Make `actix.service` file
```service
[Unit]
Description=Rust actix basics upgrade server
After=network.target
#After=mysqld.service # Uncomment when using mysql on same server
StartLimitIntervalSec=0

[Service]
Type=simple
#Restart=always
#RestartSec=1
User=ec2-user
ExecStart=/usr/bin/sh /home/status-reports/server/run.sh

[Install]
WantedBy=multi-user.target
```
- [ ] Start server

### TODO: Want

- [ ] Certbot (**Before** copy SSL cert)
```shell
sudo dnf update
sudo dnf upgrade -y
sudo dnf install python3 augeas-libs
echo -e "$Cyan Install certbot ... $Color_Off"
sudo python3 -m venv /opt/certbot/
sudo /opt/certbot/bin/pip install --upgrade pip
sudo /opt/certbot/bin/pip install certbot certbot
sudo ln -s /opt/certbot/bin/certbot /usr/bin/certbot
echo -e "$Cyan Getting ssl cert using certbot ... $Color_Off"
sudo certbot certonly --standalone
```
- [ ] Have user make config files (**Before** Start server)

## dev_*.sh

helper script for devlement