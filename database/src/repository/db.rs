use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use config::Config;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let secret_config = Config::builder()
        .add_source(config::File::with_name("secret.config.toml"))
        .build()
        .expect("Missing Secret Config File");


    let database_url = secret_config.get("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
