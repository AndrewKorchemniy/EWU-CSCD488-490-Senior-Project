# Helper Scripts

## build.sh

build all the parts

## package.sh

make a tar the files need for server

## install.sh

Install

### TODO: Need

- [ ] Install libs (`openssl`)
```shell
sudo dnf update
sudo dnf install openssl mariadb-connector-c
```
- [ ] Setup server user (`status-reports`)
```shell
sudo useradd -m -s /usr/sbin/nologin status-reports
```
- [ ] Copy server into user (`/home/status-reports/server`)
```shell
sudo su - status-reports -c "mkdir -p /home/status-reports/server"
sudo cp target/release/backend /home/status-reports/server/
sudo cp target/release/dbcli /home/status-reports/server/
sudo cp -r database/migrations /home/status-reports/server/
sudo cp helper_scripts/srs-actix.service /home/status-reports/server/
# TODO: make link (/etc/systemd/system/)
sudo cp helper_scripts/run.sh /home/status-reports/server
sudo cp -r studentpage/dist /home/status-reports/server/studentpage/
sudo cp -r adminpage/dist /home/status-reports/server/adminpage/
sudo cp -r res /home/status-reports/server/
# TODO: Wait for IT to config server
sudo cp secret.config.toml /home/status-reports/server/
sudo cp server.config.toml /home/status-reports/server/
sudo chown -R status-reports:status-reports /home/status-reports/server/
```
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
- [ ] Setup DB
Code missing
- [ ] Make `srs-actix.service` file
```unit file (systemd)
[Unit]
Description=Status Repoting System Server in Rust actix
After=network.target
#After=mysqld.service # Uncomment when using mysql on same server
StartLimitIntervalSec=0

[Service]
Type=simple
#Restart=always
#RestartSec=1
#StartLimitBurst=5
#StartLimitIntervalSec=10
User=status-reports
ExecStart=/usr/bin/sh /home/status-reports/server/run.sh

[Install]
WantedBy=multi-user.target
```
- [ ] Start server
```shell
sudo systemctl start srs-actix
sudo systemctl status actix
```

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