use serde::{Serialize, Deserialize};

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct User {
    id: Option<u32>,
    name: String
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
}