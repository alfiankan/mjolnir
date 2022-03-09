use owo_colors::OwoColorize;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap};
use std::io::{Error, Read, Write};
use std::process::exit;
use std::time::UNIX_EPOCH;
use std::{fs, io, time};

pub mod mjl;
use crate::mjl::types;

fn find_all_records() {
    println!();
    let mut file = fs::OpenOptions::new().read(true).open("data.mj").unwrap();
    let mut datas = String::new();
    file.read_to_string(&mut datas).unwrap();
    file.flush().unwrap();

    let rows: Vec<&str> = datas.trim().split("\n").collect();

    //load all box
    let mut boxs: BTreeMap<&str, &str> = BTreeMap::new();

    for x in &rows {
        let cols: Vec<&str> = x.split("|").collect();

        // find if exist remove
        match boxs.get_key_value(cols[0]) {
            None => {
                if cols.len() >= 3 {
                    boxs.insert(cols[0], cols[3]);
                }
            }
            Some(_) => {}
        };

        // if not exist insert
    }

    if boxs.len() > 0 {
        for x in boxs {
            println!("\t - {}", x.0);
        }
    }
}

fn find_last_hash(chain_key: &str) -> types::SingleChain {
    let mut file = fs::OpenOptions::new().read(true).open("data.mj").unwrap();
    let mut datas = String::new();
    file.read_to_string(&mut datas).unwrap();
    file.flush().unwrap();

    let rows: Vec<&str> = datas.trim().split("\n").collect();

    //load all box
    let mut boxs: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
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
        let box_data = match boxs.get_key_value(chain_key) {
            None => {
                exit(0);
            }
            Some(box_data) => box_data,
        };
        return types::SingleChain {
            prev_hash: Box::from(box_data.0.to_string()),
            data: Box::from(box_data.1[0]),
            hashed_data: Box::from(box_data.1[1]),
        };
    }
    println!("BOXS HEAD {:?} ", box_head);

    for x in boxs.clone() {
        println!("BOXS DATA {:?} ", x);
    }

    // traversal
    let mut last_data = types::SingleChain {
        prev_hash: Box::from(""),
        data: Box::from(""),
        hashed_data: Box::from(""),
    };
    loop {
        //let box_data = boxs.get_key_value(box_head.as_str()).unwrap();

        let box_data = match boxs.get_key_value(box_head.as_str()) {
            None => {
                break;
            }
            Some(box_data) => box_data,
        };
        last_data.prev_hash = Box::from(box_data.0.to_string());
        last_data.hashed_data = Box::from(box_data.1.to_vec()[1]);
        last_data.data = Box::from(box_data.1.to_vec()[0]);

        box_head = box_data.1.to_vec()[1].to_string();
    }
    boxs.clear();
    println!("LAST HASH {}", last_data.data);
    return last_data;
}

fn insert_chain(chain_key: &str, payload: &str) {
    let last_key = find_last_hash(chain_key).hashed_data;
    println!("INSERT TO LAST KEY {:?}", last_key);
    let data = types::DataBox {
        prev_hash: last_key.clone(),
        data: Box::from(payload),
    };
    //chain_key|prev_hash|data|hash;
    match serde_json::to_string(&data) {
        Ok(data) => {
            //chain_key|prev_hash|data|hash;
            let mut save_data = String::new();
            save_data.push_str(chain_key);
            save_data.push_str("|");
            save_data.push_str(last_key.to_string().as_str());
            save_data.push_str("|");
            save_data.push_str(data.as_str());
            save_data.push_str("|");
            save_data.push_str(hash_data(data.as_str()).as_str());
            save_data.push_str("\n");
            //save_data.push_str(hash_data(data.as_str()).as_str());
            // println!("DATA {}", base64::encode(data).as_str());
            match add_chain_to_file(save_data.as_str()) {
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
        [
            "INSERT TO :chain_key :string_data",
            "insert new chain to record",
        ],
        ["SELECT :chain_key", "get record"],
        ["SELECT :chain_key ALL", "get chains"],
        ["FIND :hash ALL", "get chains"], //TODO: this
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
            Ok(_) => match exit_confirmation.trim().to_uppercase().as_str() {
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
            },
            Err(err) => {
                println!("Error => {}", err.red());
                exit(-1);
            }
        }
    }
}

fn eval_insert(mql: &str) {
    let mql_tokens: Vec<&str> = mql.split(' ').collect();
    //println!("{} {}", mql_tokens[0].red(), mql_tokens[1].green());
    println!("KEY CHAIN {:?}", mql_tokens[2].on_red());

    if mql_tokens.len() < 4 {
        println!(
            "{}",
            "invalid mql should INSERT TO :chain_key :string_data".red()
        )
    } else {
        if mql_tokens[1].to_uppercase().as_str() == "TO" {
            println!("INSERTING TO CHAIN {}", mql_tokens[2].blue());

            let payload_indexs = Regex::new(r"'(.+?)'").unwrap().find(&mql).unwrap();

            // for i in 0..5000 {
            //     insert_chain(mql_tokens[2], i.to_string().as_str());
            // }

            insert_chain(
                mql_tokens[2],
                &mql[payload_indexs.start()..payload_indexs.end()],
            )
        } else {
            // unprocessable
            println!(
                "{}",
                "invalid mql should INSERT TO :chain_key :string_data".red()
            )
        }
    }
}
fn chained_view(prev_hash: &str, data: &str, hashed_data: &str) {
    // trim every 50 chars
    let long_text: Vec<char> = data.chars().collect();
    println!("\t{}", &prev_hash.on_blue());
    println!("\t    │    ");
    println!("\t   ─┴─   ");
    if long_text.len() > 50 {
        let mut pointer = 0;
        let mut writer_pointer = 0;
        loop {
            // print!("\t    ├──────────│ ");

            for i in (pointer * 50)..((pointer * 50) + 50) {
                if i < long_text.len() {
                    writer_pointer = writer_pointer + 1;
                    print!("{}", long_text[i].green());
                } else {
                    break;
                }
            }

            //write end
            print!(" \n");
            pointer = pointer + 1;
            if writer_pointer == long_text.len() {
                break;
            }
        }
    } else {
        //write
        for i in 0..long_text.len() {
            print!("{}", long_text[i].green());
        }
        //write end
        print!(" \n");
    }
    println!("\t   ─┬─   ");
    println!("\t    │    ");
    println!("\t{}", &hashed_data.on_blue());

    println!("\t    ▲    ");
    println!("\t    │    ");
    println!("\t    ▼    ");
}

fn eval_select(mql: &str) {
    let mql_tokens: Vec<&str> = mql.split(' ').collect();
    //println!("{} {}", mql_tokens[0].red(), mql_tokens[1].green());

    if mql_tokens.len() < 2 {
        println!("{}", "invalid mql see help".red())
    } else {
        // if selecet all
        if mql_tokens.len() == 3 {
            if mql_tokens[2].to_uppercase() == "ALL" {
                println!("get all chains");

                let mut file = fs::OpenOptions::new().read(true).open("data.mj").unwrap();
                let mut datas = String::new();
                file.read_to_string(&mut datas).unwrap();
                file.flush().unwrap();

                let rows: Vec<&str> = datas.trim().split("\n").collect();

                //load all box
                let mut boxs: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
                let mut box_head = String::new();
                for x in &rows {
                    let cols: Vec<&str> = x.split("|").collect();
                    if cols[0] == mql_tokens[1] && cols[1] == mql_tokens[1] {
                        box_head = cols[3].to_string();
                    }
                    if cols[0] == mql_tokens[1] {
                        boxs.insert(cols[1], vec![cols[2], cols[3]]);
                    }
                }
                loop {
                    //let box_data = boxs.get_key_value(box_head.as_str()).unwrap();

                    let box_data = match boxs.get_key_value(box_head.as_str()) {
                        None => {
                            break;
                        }
                        Some(box_data) => box_data,
                    };

                    chained_view(box_data.0, box_data.1.to_vec()[0], box_data.1.to_vec()[1]);
                    box_head = box_data.1.to_vec()[1].to_string();

                    //validating
                    if hash_data(box_data.1.to_vec()[0]) != box_data.1.to_vec()[1] {
                        println!("\t{}", "DATA IS NOT VALID [No]".on_red());
                        break;
                    }
                }
                boxs.clear();
            }
        } else if mql_tokens.len() == 2 {
            let record = find_last_hash(mql_tokens[1]);
            println!("{:?}", record.data.to_string().as_str());
            chained_view(
                record.prev_hash.to_string().as_str(),
                record.data.to_string().as_str(),
                record.hashed_data.to_string().as_str(),
            );
        }
    }
}

fn eval_find(mql: &str) {
    let mql_tokens: Vec<&str> = mql.split(' ').collect();
    //println!("{} {}", mql_tokens[0].red(), mql_tokens[1].green());

    if mql_tokens.len() < 2 {
        println!("{}", "invalid mql see help".red())
    } else {
        let mut file = fs::OpenOptions::new().read(true).open("data.mj").unwrap();
        let mut datas = String::new();
        file.read_to_string(&mut datas).unwrap();
        file.flush().unwrap();

        let rows: Vec<&str> = datas.trim().split("\n").collect();

        //load all box
        let mut boxs: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for x in &rows {
            let cols: Vec<&str> = x.split("|").collect();
            boxs.insert(cols[3], vec![cols[2], cols[1]]);
        }

        //find

        match boxs.get_key_value(mql_tokens[1]) {
            None => {
                println!("{} NOT FOUND", mql_tokens[1].red());
            }
            Some(box_data) => {
                chained_view(box_data.1.to_vec()[1], box_data.1.to_vec()[0], box_data.0);
                if hash_data(box_data.1.to_vec()[0]) != box_data.0.to_string() {
                    println!("\t{}", "DATA IS NOT VALID [No]".on_red());
                } else {
                    match boxs.get_key_value(box_data.1.to_vec()[1]) {
                        None => {
                            println!("\t{}", "DATA IS NOT VALID [No]".on_red());
                        }
                        Some(_) => {
                            //check hash

                            println!("\t{}", "DATA IS VALID [Ok]".on_green());
                        }
                    };
                }
            }
        };
    }
}

fn add_chain_to_file(data: &str) -> Result<(), Error> {
    let mut file = match fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.mj")
    {
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
            let data = types::DataBox {
                prev_hash: Box::from(prev_hash),
                data: Box::from("{}"),
            };

            match serde_json::to_string(&data) {
                Ok(data) => {
                    let chain_key = hash_data(singularity.as_nanos().to_string().as_str());
                    println!("chain key: {}", chain_key);

                    //chain_key|prev_hash|data|hash;
                    let mut save_data = String::new();
                    save_data.push_str(chain_key.as_str());
                    save_data.push_str("|");
                    save_data.push_str(chain_key.as_str());
                    save_data.push_str("|");
                    save_data.push_str(data.as_str());
                    save_data.push_str("|");
                    save_data.push_str(hash_data(data.as_str()).as_str());
                    save_data.push_str("\n");

                    //save_data.push_str(hash_data(data.as_str()).as_str());
                    // println!("DATA {}", base64::encode(data).as_str());
                    match add_chain_to_file(save_data.as_str()) {
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
    let mql_tokens: Vec<&str> = mql.split(' ').collect();

    match mql_tokens[0].to_uppercase().as_str() {
        "GEN" => {
            println!("GENERATE NEW CHAIN");
            genesis();
        }
        "RECORDS" => {
            find_all_records();
        }
        "INSERT" => {
            eval_insert(mql);
        }
        "SELECT" => {
            eval_select(mql);
        }
        "FIND" => {
            eval_find(mql);
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
