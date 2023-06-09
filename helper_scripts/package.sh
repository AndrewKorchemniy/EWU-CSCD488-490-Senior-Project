#tar.gz
# Executable: backend, dbcli
# Database: migrations
# Scripts: service, run
# Front-end (static): studentpage adminpage res
# Config: secret.config server.config
tar -czvf bin.tar.gz target/release/backend target/release/dbcli database/migrations helper_scripts/srs-actix.service helper_scripts/run.sh studentpage/dist adminpage/dist res secret.config.toml server.config.toml