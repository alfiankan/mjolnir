use std::{fs, io, time};
use std::io::{Read, Write};
use base64_stream::FromBase64Reader;
use regex::internal::Char;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::UNIX_EPOCH;
use owo_colors::OwoColorize;


#[test]
fn trim_mql() {
    let mql = "insert to f545 'dfffdf fsf fsf'";

    // let mut mql_char: Vec<Char> = Vec::new();
    // for x in mql.chars() {
    //     mql_char.push(Char::from(x));
    // }

    // char_vector.iter().cloned().collect::<String>();
    println!("{}",  &mql[1..3]);

}

#[test]
fn write_line_chain() {

    let mut file = fs::OpenOptions::new().write(true).append(true).open("data.mj").unwrap();
    file.write_all(b"hello 1\n").unwrap();
    file.flush().unwrap();
}

#[test]
fn find_last_hash() {
    let mut file = fs::OpenOptions::new().read(true).open("data.mj").unwrap();
    let mut datas = String::new();
    file.read_to_string(&mut datas).unwrap();
    file.flush().unwrap();
    //println!("{}", datas);
    // for x in datas.split(";") {
    //     println!("==> {}", x);
    // }
    let rows: Vec<&str> = datas.trim().split("\n").collect();
    //println!("{:?}", rows);

    //load all box
    let mut boxs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut box_head = String::new();
    for x in rows {
        let cols: Vec<&str> = x.split("|").collect();
        if cols[0] == "8f90cd15532ab112528c017ac452f7adc1072f16b25927babc8b5697fd614307".to_string().as_str() && cols[1] == "8f90cd15532ab112528c017ac452f7adc1072f16b25927babc8b5697fd614307".to_string().as_str() {
            box_head = cols[3].to_string();
        }
        if cols[0] == "8f90cd15532ab112528c017ac452f7adc1072f16b25927babc8b5697fd614307".to_string().as_str() {
            boxs.insert(cols[1], vec![cols[2], cols[3]]);
        }
    }

    println!("CHAINS LENGTH {}", boxs.len());
    println!("HEAD {}", box_head.blue());

    //println!("{:?}", boxs.get_key_value("7073d6f717c529e2ede1330545a7edd28f07aecbaf28e485e00917ba388522cf") );

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
        print!("{}[2J", 27 as char);
        println!("=============================");
        println!("PREV_HASH {}", box_data.0.green());
        println!("DATA {}", box_data.1.to_vec()[0].on_green());
        println!("HASH {}", box_data.1.to_vec()[1].green());
        last_hash = box_data.1.to_vec()[1].to_string();

        box_head = box_data.1.to_vec()[1].to_string();
        sleep(time::Duration::from_millis(20));
    }
    println!("CHAINS LENGTH {}", boxs.len().on_green());
    boxs.clear();
    println!("LAST HASH {}", last_hash.purple())
}

fn find_last(chain_key: &str) -> String {

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
        println!("only one box");
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

#[test]
fn find_last_hash_last() {
    println!("LAST {:?}", time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
}
