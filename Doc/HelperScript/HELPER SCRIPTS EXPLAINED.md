

## build.sh

build all the parts needed to run the website. the backend, database cli, admin page and student page. 

## build_code_doc.sh

this creates documentation with the notes in the program created with the 3 lines "///".

## clean.sh

this is the make file clean by removing all the files we build from. the files are backend, database cli, admin page and student page. useful when the build doesn't build for the new code.

## dev_db_setup.sh

this creates one row for all the tables in the database for one student with the sprint number, the team name, there own name, and there email.

to rerun this script you will need to delete the rows or drop the tables and re-up the tables  before re running this code.

this is the basic setup for one student before they fill in there report.

## dev_run_local.sh

this is the the setup build script to run server on locally. you may need to update the location of the repo on line 8.

## dev_run_on_aws.sh

This is the setup build script to run server on the AWS (Amazon Linux 2003). also check and make sure to update location(line8) is correct. this doesn't work with the AWS we need(it lied to us saying that it would).

## dev_rust_setup.sh

update package manager(dnf) on operating system. install the dev tools from the package manager(dnf). install openssl dev tools and mysql client dev tools. then install rustup. then install clippy. add a target to compile to the web. then install the Trunk tool from Cargo. Install the diesel dev tools. then you clone the project from the repo. 

## lint.sh

this checks if your code passes the linter(which is the clippy and format).

## lint_warning.sh

this gives you the full report of the code mistakes 

## package.sh

make a tar the files need for server

## run.sh

runs the backend (mainly meant for server side only)

## install.sh

Install(currently not implemented)

### TODO: Need

- [ ] Install libs (`openssl`)
```shell
sudo yum update
# TODO: get openssl 3 
sudo yum install openssl openssl-libs
sudo amazon-linux-extras enable mariadb10.5
sudo yum clean metadata
sudo yum install mariadb-libs
```
> mariadb-connector-c ?
- [ ] Setup server user (`status-reports`)
```shell
sudo useradd -m status-reports
```
- [ ] Get server
```shell
# TODO: download https://github.com/tztz8/EWU-CSCD488-490-Senior-Project/releases
# unzip
tar -xvzf bin.tar.gz
```
- [ ] Copy server into user (`/home/status-reports/server`)
```shell
echo "Make program folder"
sudo su - status-reports -c "mkdir -p /home/status-reports/server"
echo "Executables"
sudo cp target/release/backend /home/status-reports/server/
sudo cp target/release/dbcli /home/status-reports/server/
echo "SQL setup files"
sudo cp -r database/migrations /home/status-reports/server/
echo "File to allow run in the background"
sudo cp helper_scripts/srs-actix.service /home/status-reports/server/
sudo cp helper_scripts/run.sh /home/status-reports/server
echo "Static files"
sudo cp -r studentpage/dist /home/status-reports/server/studentpage/dist
sudo cp -r adminpage/dist /home/status-reports/server/adminpage/dist
sudo cp -r res /home/status-reports/server/
echo "Config files"
# TODO: Wait for IT to config server
sudo cp secret.config.toml /home/status-reports/server/
sudo cp server.config.toml /home/status-reports/server/
echo "Set file permissions"
sudo chown -R status-reports:status-reports /home/status-reports/server/
echo "Allow/link service"
sudo systemctl link /home/status-reports/server/srs-actix.service
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
- [ ] Lock server user
```shell
sudo usermod -s /usr/sbin/nologin status-reports
```
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
sudo systemctl status srs-actix
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

