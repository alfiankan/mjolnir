
use sha2::{Digest, Sha256};
use crate::types::DataBox;

pub fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    return hex::encode(result);
}

/// serialize box to string
/// return empty string if error
pub fn serializer(prev_hash: &str, data: DataBox, key_chain: &str) -> String {
    match serde_json::to_string(&data) {
        Ok(data) => {

            // row format : chain_key|prev_hash|data|hash
            let mut save_data = String::new();
            save_data.push_str(key_chain);
            save_data.push_str("|");
            save_data.push_str(prev_hash);
            save_data.push_str("|");
            save_data.push_str(data.as_str());
            save_data.push_str("|");
            save_data.push_str(hash_data(data.as_str()).as_str());
            save_data.push_str("\n");
            return save_data;
        }
        Err(_) => {
            // TODO: handle log
            return "".to_string();
        }
    }
}

#[test]
fn test_hash() {
    println!("{}", hash_data("hello world"));
}

#[test]
fn test_serialize() {
    let some_box = DataBox{
        prev_hash: Box::from("xxxxx"),
        data: Box::from("yyyyy")
    };
    println!("{}", serializer("sssss",some_box, "someky") );
}