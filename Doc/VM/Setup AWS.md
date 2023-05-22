# Setup AWS

Notes I made as I setup on AWS EC2 Amazon linux

## SSL Cert:

1. Install python3

   ```shell
   sudo dnf install python3 augeas-libs
   ```

2. Setup/install certbot

   ```shell
   sudo python3 -m venv /opt/certbot/
   sudo /opt/certbot/bin/pip install --upgrade pip
   sudo /opt/certbot/bin/pip install certbot certbot
   sudo ln -s /opt/certbot/bin/certbot /usr/bin/certbot
   ```

3. Get cert

   > Note: need email
   >
   > Note: domain needs to be setup
   >
   > Note: no server running on both 80 and 443 when running this command (but server can go online after)

   ```shell
   sudo certbot certonly --standalone
   ```

4. Write down cer and key file locashon:

   Ex.

   ```toml
   SSL_CERTIFICATE="/etc/letsencrypt/live/aws.tftinker.tech/fullchain.pem"
   SSL_KEY="/etc/letsencrypt/live/aws.tftinker.tech/privkey.pem"
   ```

### Monthly

Update certbot run

```shell
sudo /opt/certbot/bin/pip install --upgrade certbot
```

Every 3 months run to update cert (cert expire every 3 months)

```shell
sudo certbot renew -q
```

## Setup dev code to executable

get gcc and it libs

```shell
sudo dnf groupinstall 'Development Tools'
sudo dnf install openssl-devel
```

get rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run executable

TODO: add

