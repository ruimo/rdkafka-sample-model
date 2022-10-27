use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_name: String,
    pub email: String,
}

impl User {
    pub fn deserialize(bytes: &[u8]) -> Result<User, bincode::Error> {
        bincode::deserialize(bytes)
    }

    pub fn serialize(user: &User) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(user)
    }
}