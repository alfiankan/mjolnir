use serde::{Deserialize, Serialize};
use std::any::TypeId;
use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBox {
    pub prev_hash: Box<str>,
    pub data: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleBox {
    pub prev_hash: Box<str>,
    pub data: Box<str>,
    pub hashed_data: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectResult {
    pub total_data: i64,
    pub total_valid_data: i64,
    pub boxs: Vec<SingleBox>,
}

pub struct MjlError {
    details: String,
}

impl MjlError {
    pub fn new(msg: &str) -> MjlError {
        return MjlError {
            details: msg.to_string(),
        };
    }
}

impl Debug for MjlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Display for MjlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MjlError {
    fn description(&self) -> &str {
        return &self.details;
    }
}
