#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod usersModel;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let user = usersModel::NewUser {
        username: String::from("Arcuzam"),
        password: String::from("macmon66"),
        email: String::from("suarez0550@gmail.com")
    };

    if usersModel::User::insert(user, &conn) {
        println!("success");
    } else {
        println!("failed");
    }
}