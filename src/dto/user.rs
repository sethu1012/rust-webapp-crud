use serde::{Serialize, Deserialize};

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct User {
    id: Option<u32>,
    name: String,
    email: String,
    password: String
}

impl User {
    pub fn get_id(&self) -> u32 {
        match self.id {
            Some(id) => id,
            None => 0
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }
}