use crate::mjl::utils::{hash_data, serializer};
use crate::types::DataBox;
use owo_colors::OwoColorize;
use std::borrow::Borrow;
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
    fn new(file_name: &str) -> Engine {
        return Engine {
            file_name: file_name.to_string(),
        };
    }

    /// read data from file as rows
    fn read_persistent_data(&self) -> Result<String, bool> {
        let mut data = String::new();

        let mut file = match fs::OpenOptions::new()
            .read(true)
            .open(self.file_name.as_str())
        {
            Ok(file) => file,
            Err(e) => {
                // TODO: handle log errors
                println!("ERROR {}", e.on_red());
                return Err(false);
            }
        };

        return match file.read_to_string(&mut data) {
            Ok(_) => Ok(data),
            Err(e) => {
                // TODO: handle log errors
                println!("ERROR {}", e.on_red());
                Err(false)
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
                // TODO: handle error
                println!("Problem opening file: {:?}", error);
                false
            }
        };
    }

    /// genesis new record
    pub fn genesis(&self) -> Result<bool, String> {
        return match time::SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(singularity) => {
                // gen very begining box
                let prev_hash = hash_data(singularity.as_nanos().to_string().as_str());
                let first_data_box = DataBox {
                    prev_hash: Box::from(prev_hash.clone()),
                    data: Box::from("{}"),
                };

                let serialized = serializer(first_data_box, prev_hash.borrow());
                if serialized == "" {
                    // TODO: handle err
                    Err("genesis chain failed".to_string())
                } else {
                    // writing to file
                    println!("{}", serialized.on_green());
                    if self.write_persistent_data(serialized) {
                        Ok(true)
                    } else {
                        Err("genesis chain failed".to_string())
                    }
                }
            }
            Err(_) => {
                Err("genesis chain failed {}".to_string())
            }
        }
    }
}



#[test]
fn read_persistent_file_test() {
    let mjl = Engine::new("data.mj");

    let result = mjl.read_persistent_data();

    println!("{:?}", result);

    match result {
        Ok(_) => {
            assert_eq!(true, true);
        }
        Err(_) => {
            assert_eq!(true, false);
        }
    }
}

#[test]
fn genesis_test() {
    let mjl = Engine::new("data.mj");

    mjl.genesis();
}

#[test]
fn insert_to_chain() {

}

#[test]
fn get_last_databox() {}

#[test]
fn verify_valid_chain() {}

#[test]
fn get_all_databox_as_chain() {}

#[test]
fn read_persistent_file() {

}
