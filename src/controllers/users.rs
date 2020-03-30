use rocket::request::Form;
use rocket::http::RawStr;
use rocket_contrib::json::{Json};

#[path = "../dto/mod.rs"]
mod dto;
use dto::user;
use crate::DbConn;
use crate::diesel::RunQueryDsl;
use crate::schema::users::dsl::*;

#[path = "../models/mod.rs"]
mod models;
use models::user::User;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::ExpressionMethods;

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}

#[get("/test")]
pub fn test(conn: DbConn) {
    let results = users.load::<User>(&*conn).map_err(|err| -> String {
        println!("Error inserting row: {:?}", err);
        "Error inserting row into database".into()
    });

    for user in results {
        println!("{:?}", user);
    }
}

#[post("/login?<_email>&<_password>")]
pub fn login(conn: DbConn, _email: &RawStr, _password: &RawStr) {
    let _result = users.filter(email.eq(_email.as_str())).first::<User>(&*conn).map_err(|err| -> String {
        format!("{:?}", err)
    });

    match _result {
        Ok(_) => {
            let _user: User = _result.unwrap();
            if format!("{:x}", md5::compute(_password.as_str())) == _user.password {
                println!("{}", "Matched");
            }
        },
        Err(_result) => println!("{:?}", _result)
    };
}

#[post("/form", format = "json", data = "<user>")]
pub fn create_user(user: Json<user::User>) {
    let form_data: user::User = user.into_inner();
    println!("{:?}", String::from(form_data.get_id().to_string()));
    println!("{:?}", String::from(form_data.get_name()));
    println!("{:?}", form_data);
}

#[post("/create", data = "<user>")]
pub fn form_user(conn: DbConn, user: Form<user::User>) {
    let form_data: user::User = user.into_inner();
    let new_user: User = 
        User { 
            id: None,  
            name: String::from(form_data.get_name()), 
            email: String::from(form_data.get_email()), 
            password: format!("{:x}", md5::compute(String::from(form_data.get_password())))
        };
    
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&*conn)
        .expect("Could not save user");
}