use std::{fs, io, time};
use std::collections::HashMap;
use std::io::{Error, Read, Write};
use std::process::exit;
use std::time::{UNIX_EPOCH};
use owo_colors::OwoColorize;
use regex::Regex;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug)]
struct DataBox {
    prev_hash: Box<str>,
    data: Box<str>,
}




fn find_last_hash(chain_key: &str) -> String {

    let mut file = fs::OpenOptions::new().read(true).open("data.mj").unwrap();
    let mut datas = String::new();
    file.read_to_string(&mut datas).unwrap();
    file.flush().unwrap();

    let rows: Vec<&str> = datas.trim().split("\n").collect();

    //load all box
    let mut boxs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut box_head = String::new();
    for x in &rows {
        let cols: Vec<&str> = x.split("|").collect();
        if cols[0] == chain_key && cols[1] == chain_key {
            box_head = cols[3].to_string();

        }
        if cols[0] == chain_key {
            boxs.insert(cols[1], vec![cols[2], cols[3]]);
        }
    }

    if boxs.len() == 1 {
        let cols: Vec<&str> = rows[0].split("|").collect();
        return cols[3].to_string();
    }

    // traversal
    let mut last_hash = String::new();
    loop {
        //let box_data = boxs.get_key_value(box_head.as_str()).unwrap();

        let mut box_data = match boxs.get_key_value(box_head.as_str()) {
            None => {
                break;
            }
            Some(box_data) => box_data
        };
        last_hash = box_data.1.to_vec()[1].to_string();

        box_head = box_data.1.to_vec()[1].to_string();
    }
    boxs.clear();
    //println!("LAST HASH {}", last_hash.purple())
    return last_hash;
}

fn insert_chain(chain_key: &str, payload: &str) {

    let last_key = find_last_hash(chain_key);
    let data = DataBox{
        prev_hash: Box::from(last_key.as_str()),
        data: Box::from(payload),
    };
    //chain_key|prev_hash|data|hash;
    match serde_json::to_string(&data) {
        Ok(data) => {

            //chain_key|prev_hash|data|hash;
            let mut save_data = String::new();
            save_data.push_str(chain_key);
            save_data.push_str("|");
            save_data.push_str(last_key.as_str());
            save_data.push_str("|");
            save_data.push_str(data.as_str());
            save_data.push_str("|");
            save_data.push_str(hash_data( data.as_str() ).as_str());
            save_data.push_str("\n");
            //save_data.push_str(hash_data(data.as_str()).as_str());
            // println!("DATA {}", base64::encode(data).as_str());
            match add_chain_to_file(save_data.as_str() ) {
                Ok(_) => {
                    //println!("record saved");
                }
                Err(e) => {
                    println!("{}", e);
                }
            }

        }
        Err(err) => {
            println!("Serializer error {}", err.red());
        }
    }


}



fn show_help() {
    let helps = [
        ["GEN", "genesis new record, return chain_key"],
        ["INSERT TO :chain_key :string_data", "insert new chain to record"],
        ["SELECT :chain_key", "get record"],
        ["SELECT :chain_key ALL", "get chain history limit to 50 chain"]
    ];
    for help in helps {
        print!("{}", help[0].on_red());
        print!("{}", " => ");
        print!("{}", help[1]);
        print!("{}", "\n");
    }
}

fn show_exit_confirmation() {
    let input = io::stdin();
    let mut exit_confirmation = String::new();
    loop {
        print!("Are you sure ? [y/N] ");
        io::stdout().flush().expect("Fatal Error");

        match input.read_line(&mut exit_confirmation) {
            Ok(_) => {
                match exit_confirmation.trim().to_uppercase().as_str() {
                    "Y" => {
                        exit(-1);
                    }
                    "N" => {
                        println!("cancel exit");
                        break;
                    }
                    &_ => {
                        println!("cancel exit");
                        break;
                    }
                }
            }
            Err(err) => {
                println!("Error => {}", err.red());
                exit(-1);
            }
        }
    }
}

fn eval_insert(mql: &str) {
    let mql_tokens: Vec<&str> = mql.split( ' ').collect();
    //println!("{} {}", mql_tokens[0].red(), mql_tokens[1].green());


    if mql_tokens.len() < 4 {
            println!("{}","invalid mql should INSERT TO :chain_key :string_data".red())

    } else {
        if mql_tokens[1].to_uppercase().as_str() == "TO" {
            println!("INSERTING TO CHAIN {}", mql_tokens[2].blue());

            let payload_indexs = Regex::new(r"'(.+?)'").unwrap().find(&mql).unwrap();

            for i in 0..5000 {
                insert_chain(mql_tokens[2], i.to_string().as_str());
            }

            //insert_chain(mql_tokens[2], &mql[ payload_indexs.start()..payload_indexs.end() ])

        } else {
            // unprocessable
            println!("{}","invalid mql should INSERT TO :chain_key :string_data".red())
        }

    }

}

fn add_chain_to_file(data: &str) -> Result<(), Error> {
    let mut file = match fs::OpenOptions::new().write(true).append(true).open("data.mj") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    return match file.write_all(data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

}

fn hash_data(data: &str) -> String {

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    return hex::encode(result.clone());
}

fn genesis() {


    match time::SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(singularity) => {
            println!("<> {}", singularity.as_nanos().to_string());
            //genesis

            let prev_hash = hash_data(singularity.as_nanos().to_string().as_str());
            let data = DataBox{
                prev_hash: Box::from(prev_hash),
                data: Box::from("{}"),
            };

            match serde_json::to_string(&data) {
                Ok(data) => {
                    let chain_key = hash_data(singularity.as_nanos().to_string().as_str());
                    println!("chain key: {}",  chain_key);

                    //chain_key|prev_hash|data|hash;
                    let mut save_data = String::new();
                    save_data.push_str(chain_key.as_str());
                    save_data.push_str("|");
                    save_data.push_str(chain_key.as_str());
                    save_data.push_str("|");
                    save_data.push_str(data.as_str());
                    save_data.push_str("|");
                    save_data.push_str(hash_data( data.as_str() ).as_str());
                    save_data.push_str("\n");

                    //save_data.push_str(hash_data(data.as_str()).as_str());
                    // println!("DATA {}", base64::encode(data).as_str());
                    match add_chain_to_file(save_data.as_str() ) {
                        Ok(_) => {
                            // insert empty box
                            println!("new record saved");
                            println!("KEY CHAIN {}", chain_key.green());
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }

                }
                Err(err) => {
                    println!("Serializer error {}", err.red());
                }
            }

        }
        Err(err) => {
            println!("error {}", err.red())
        }
    }
}

fn eval_command(mql: &str) {
    let mql_tokens: Vec<&str> = mql.split( ' ').collect();

    match mql_tokens[0].to_uppercase().as_str() {
        "GENESIS" => {
            println!("GENERATE NEW CHAIN");
            genesis();
        }
        "INSERT" => {
            eval_insert(mql);
        }
        "HELP" => {
            show_help();
        }
        "EXIT" => {
            show_exit_confirmation();
        }
        &_ => {
            println!("{} is not mql command; run 'help' to get help", mql.red());
        }
    }
}

fn main() {
    // mql => mjolnir query language
    let mut mql = String::new();

    let input = io::stdin();
    let mut lines = 1;
    loop {
        mql.clear();
        println!();
        print!("[{}] [mql] > ", lines);
        io::stdout().flush().expect("Fatal Error");

        match input.read_line(&mut mql) {
            Ok(_) => {}
            Err(err) => {
                println!("Error => {}", err);
                continue;
            }
        }
        eval_command(mql.trim());
        lines = lines + 1;
    }


}
