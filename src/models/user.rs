use crate::schema::users;
use serde::{Serialize, Deserialize};

#[table_name = "users"]
#[derive(Queryable, PartialEq, Insertable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub password: String
}