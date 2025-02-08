use std::num::ParseIntError;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use serde_json::Value;
use serde_json::json;

use crate::utils::generate_token;

const TOKEN_LENGTH: usize = 16;

pub trait CheshireDBSchema {
    fn get_id(&self) -> Option<RecordId>;
}

pub trait CheshireDBSchemaGetU32Id: CheshireDBSchema {
    fn uid(&self) -> Result<u32, ParseIntError> {
        let mut uid = self.get_id().unwrap().key().to_string();
        uid.retain(|c| !['⟨', '⟩'].contains(&c));

        uid.parse::<u32>()
    }
}
impl<T: CheshireDBSchema> CheshireDBSchemaGetU32Id for T {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<RecordId>,
    pub device_id: String,
    pub token: String,
}

impl Account {
    pub fn new(device_id: String) -> Self {
        Self {
            id: None,
            device_id,
            token: generate_token(TOKEN_LENGTH),
        }
    }
}

impl CheshireDBSchema for Account {
    fn get_id(&self) -> Option<RecordId> {
        self.id.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: Option<RecordId>,
    pub is_banned: bool,
    pub data: Value,
}

impl Player {
    pub fn new() -> Self {
        Self {
            id: None,
            is_banned: false,
            data: json!({}),
        }
    }
}

impl CheshireDBSchema for Player {
    fn get_id(&self) -> Option<RecordId> {
        self.id.clone()
    }
}
