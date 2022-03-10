use crate::mjl::utils::{hash_data, serializer};
use crate::types;
use crate::types::{DataBox, MjlError, SelectResult, SingleBox};
use owo_colors::OwoColorize;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{Read, Write};
use std::time::UNIX_EPOCH;
use std::{fs, time};

/// Engine
pub struct Engine {
    file_name: String,
}

impl Engine {
    /// new Engine Instance
    pub(crate) fn new(file_name: &str) -> Engine {
        return Engine {
            file_name: file_name.to_string(),
        };
    }

    /// read data from file as rows
    pub(crate) fn read_persistent_data(&self) -> Result<String, MjlError> {
        let mut data = String::new();

        let mut file = match fs::OpenOptions::new()
            .read(true)
            .open(self.file_name.as_str())
        {
            Ok(file) => file,
            Err(e) => {
                panic!("{}", e);
            }
        };

        return match file.read_to_string(&mut data) {
            Ok(_) => Ok(data),
            Err(e) => {
                return Err(MjlError::new(e.description()));
            }
        };
    }

    /// write persistent data
    fn write_persistent_data(&self, data: String) -> bool {
        let mut file = match fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.file_name.as_str())
        {
            Ok(file) => file,
            Err(error) => panic!("Problem opening file: {:?}", error),
        };

        return match file.write_all(data.as_bytes()) {
            Ok(_) => true,
            Err(error) => {
                println!("Problem opening file: {:?}", error);
                false
            }
        };
    }

    /// find last box in chain
    /// cli SELECT :key_chain
    pub fn find_last_box(&self, key_chain: String) -> Result<SelectResult, MjlError> {
        let data = match self.read_persistent_data() {
            Ok(data) => data,
            Err(e) => {
                return Err(MjlError::new("Error reading file"));
            }
        };
        let rows: Vec<&str> = data.trim().split("\n").collect();

        //load all box to Btree
        let mut boxs: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        // define head
        let mut box_head = String::new();
        for x in &rows {
            let cols: Vec<&str> = x.split("|").collect();
            if cols[0] == key_chain.as_str() && cols[1] == key_chain.as_str() {
                box_head = cols[3].to_string();
            }
            if cols[0] == key_chain.as_str() {
                boxs.insert(cols[1], vec![cols[2], cols[3]]);
            }
        }

        if boxs.len() == 1 {
            let box_data = match boxs.get_key_value(key_chain.as_str()) {
                None => {
                    return Err(MjlError::new("null record chain"));
                }
                Some(box_data) => box_data,
            };

            return Ok(SelectResult {
                total_data: boxs.len() as i64,
                total_valid_data: if hash_data(box_data.1.to_vec()[0]) != box_data.1.to_vec()[1] {
                    0
                } else {
                    1
                },
                boxs: vec![SingleBox {
                    prev_hash: Box::from(box_data.0.to_string()),
                    data: Box::from(box_data.1[0]),
                    hashed_data: Box::from(box_data.1[1]),
                }],
            });
        }

        let mut last_data = types::SingleBox {
            prev_hash: Box::from(""),
            data: Box::from(""),
            hashed_data: Box::from(""),
        };

        let mut total_valid_data: i64 = 0;
        // validate head
        match boxs.get_key_value(key_chain.as_str()) {
            None => {}
            Some(box_data) => {
                if hash_data(box_data.1.to_vec()[0]) == box_data.1.to_vec()[1] {
                    total_valid_data = total_valid_data + 1;
                }
            }
        };

        // traversal box to box over chain
        loop {
            let box_data = match boxs.get_key_value(box_head.as_str()) {
                None => {
                    break;
                }
                Some(box_data) => box_data,
            };
            // validate box after head
            if hash_data(box_data.1.to_vec()[0]) == box_data.1.to_vec()[1] {
                total_valid_data = total_valid_data + 1;
            }
            last_data.prev_hash = Box::from(box_data.0.to_string());
            last_data.hashed_data = Box::from(box_data.1.to_vec()[1]);
            last_data.data = Box::from(box_data.1.to_vec()[0]);
            box_head = box_data.1.to_vec()[1].to_string();
        }

        return Ok(SelectResult {
            total_data: boxs.len() as i64,
            total_valid_data: total_valid_data,
            boxs: vec![last_data],
        });
    }

    /// genesis new record
    /// cli GEN
    pub fn genesis(&self) -> Result<String, MjlError> {
        return match time::SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(singularity) => {
                // gen very begining box
                let prev_hash = hash_data(singularity.as_nanos().to_string().as_str());
                let first_data_box = DataBox {
                    prev_hash: Box::from(prev_hash.clone()),
                    data: Box::from("{}"),
                };

                let serialized = serializer(prev_hash.borrow(), first_data_box, prev_hash.borrow());
                if serialized == "" {
                    panic!("{}", "genesis chain failed".to_string())
                } else {
                    // writing to file

                    if self.write_persistent_data(serialized) {
                        Ok(prev_hash)
                    } else {
                        return Err(MjlError::new("genesis chain failed"));
                    }
                }
            }
            Err(e) => {
                return Err(MjlError::new(e.description()));
            }
        };
    }

    /// insert new box to chain
    /// cli INSERT TO :key_chain :data
    pub fn insert_to_chain(&self, data: String, key_chain: &str) -> Result<(), MjlError> {
        let last_key = match self.find_last_box(key_chain.to_string()) {
            Ok(last_key) => last_key,
            Err(e) => return Err(MjlError::new(e.description())),
        };

        // check is key_chain exist
        if last_key.boxs[0].hashed_data.to_string().as_str() == "" {
            return Err(MjlError::new("key_chain not exist, genesis first"));
        }

        // create new box
        let data = DataBox {
            prev_hash: Box::from(last_key.boxs[0].hashed_data.to_string().as_str()),
            data: Box::from(data),
        };

        let serialized = serializer(
            last_key.boxs[0].hashed_data.to_string().as_str(),
            data,
            key_chain,
        );
        if serialized == "" {
            return Err(MjlError::new("genesis chain failed, serializer failed"));
        } else {
            // writing to file
            if self.write_persistent_data(serialized) {
                Ok(())
            } else {
                return Err(MjlError::new("genesis chain failed, write file failed"));
            }
        }
    }

    /// cli SELECT :key_chain ALL
    pub fn get_all_box_from_record(&self, key_chain: String) -> Result<SelectResult, MjlError> {
        let data = match self.read_persistent_data() {
            Ok(data) => data,
            Err(e) => {
                return Err(MjlError::new(e.description()));
            }
        };
        let rows: Vec<&str> = data.trim().split("\n").collect();

        //load all box to Btree
        let mut boxs: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        // define head
        let mut box_head = String::new();
        for x in &rows {
            let cols: Vec<&str> = x.split("|").collect();
            if cols[0] == key_chain.as_str() && cols[1] == key_chain.as_str() {
                box_head = cols[3].to_string();
            }
            if cols[0] == key_chain.as_str() {
                boxs.insert(cols[1], vec![cols[2], cols[3]]);
            }
        }

        let mut all_box_from_record: Vec<SingleBox> = Vec::new();
        if boxs.len() == 1 {
            let box_data = match boxs.get_key_value(key_chain.as_str()) {
                None => {
                    return Err(MjlError::new("null record chain"));
                }
                Some(box_data) => box_data,
            };

            return Ok(SelectResult {
                total_data: boxs.len() as i64,
                total_valid_data: if hash_data(box_data.1.to_vec()[0]) != box_data.1.to_vec()[1] {
                    0
                } else {
                    1
                },
                boxs: vec![SingleBox {
                    prev_hash: Box::from(box_data.0.to_string()),
                    data: Box::from(box_data.1[0]),
                    hashed_data: Box::from(box_data.1[1]),
                }],
            });
        }

        let mut total_valid_data: i64 = 0;
        // validate head
        match boxs.get_key_value(key_chain.as_str()) {
            None => {}
            Some(box_data) => {
                if hash_data(box_data.1.to_vec()[0]) == box_data.1.to_vec()[1] {
                    all_box_from_record.push(SingleBox {
                        prev_hash: Box::from(box_data.0.to_string()),
                        data: Box::from(box_data.1[0]),
                        hashed_data: Box::from(box_data.1[1]),
                    });
                    total_valid_data = total_valid_data + 1;
                }
            }
        };

        // traversal box to box over chain
        loop {
            let box_data = match boxs.get_key_value(box_head.as_str()) {
                None => {
                    break;
                }
                Some(box_data) => box_data,
            };
            // validate box after head
            if hash_data(box_data.1.to_vec()[0]) == box_data.1.to_vec()[1] {
                all_box_from_record.push(SingleBox {
                    prev_hash: Box::from(box_data.0.to_string()),
                    data: Box::from(box_data.1[0]),
                    hashed_data: Box::from(box_data.1[1]),
                });
                total_valid_data = total_valid_data + 1;
            }
            box_head = box_data.1.to_vec()[1].to_string();
        }

        return Ok(SelectResult {
            total_data: boxs.len() as i64,
            total_valid_data: total_valid_data,
            boxs: all_box_from_record,
        });
    }

    /// cli RECORDS
    pub fn list_records() {}
}
