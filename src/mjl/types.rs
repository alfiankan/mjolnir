use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBox {
    pub prev_hash: Box<str>,
    pub data: Box<str>,
}

pub struct SingleChain {
    pub prev_hash: Box<str>,
    pub data: Box<str>,
    pub hashed_data: Box<str>,
}