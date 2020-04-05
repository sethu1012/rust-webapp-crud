#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate rocket_contrib;
#[macro_use] 
extern crate diesel;

use dotenv::dotenv;

mod controllers;
use controllers::users;

mod schema;

#[database("mysql_db")]
pub struct DbConn(diesel::MysqlConnection);

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/user", routes![
            users::ping,
            users::test,
            users::create_user,
            users::form_user,
            users::login
        ])
        .attach(DbConn::fairing())
        .launch();
}