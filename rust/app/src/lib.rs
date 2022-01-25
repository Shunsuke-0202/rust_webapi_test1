extern crate dotenv;

use std::env;
use dotenv::dotenv;

use diesel::mysql::MysqlConnection;
use diesel::Connection;

//create db connection object
pub fn establish_connection()->MysqlConnection{
    dotenv().ok();  //read .env file

    let database_url=env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}",database_url))
}