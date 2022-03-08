use std::{fs, io, time};
use std::io::{Read, Write};
use base64_stream::FromBase64Reader;
use regex::internal::Char;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::UNIX_EPOCH;
use owo_colors::OwoColorize;
use std::collections::BTreeMap;


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

fn chained_view(prev_hash: &str, data: &str, hashed_data: &str) {
    // trim every 50 chars
    let mut long_text: Vec<char> = data.chars().collect();
    println!("\t{}", &prev_hash[0..9].to_string().on_blue() );
    println!("\t    │    ");
    if long_text.len() > 50 {
        let mut pointer = 0;
        let mut writer_pointer = 0;
        loop {
            print!("\t    ├──────────│ ");

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
        print!("\t    ├──────────│ ");
        for i in 0..long_text.len() {
            print!("{}", long_text[i].green());
        }
        //write end
        print!(" \n");

    }
    println!("\t    │    ");
    println!("\t{}", &hashed_data[0..9].to_string().on_blue() );

    println!("\t    ▲    ");
    println!("\t    │    ");
    println!("\t    ▼    ");
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
    let mut boxs: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
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

        let box_data = match boxs.get_key_value(box_head.as_str()) {
            None => {
                break;
            }
            Some(box_data) => box_data
        };
        //print!("\x1B[2J\x1B[1;1H");

        // println!("=============================");
        // println!("PREV_HASH {}", box_data.0.green());
        // println!("DATA {}", box_data.1.to_vec()[0].on_green());
        // println!("HASH {}", box_data.1.to_vec()[1].green());
        //
        chained_view(box_data.0, box_data.1.to_vec()[0], box_data.1.to_vec()[1]);

        last_hash = box_data.1.to_vec()[1].to_string();

        box_head = box_data.1.to_vec()[1].to_string();
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

        let box_data = match boxs.get_key_value(box_head.as_str()) {
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

#[test]
fn chain_display() {
    println!("LAST {:?}", time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
    println!();

    for _ in 0..5 {

        // trim every 20 chars
        //let mut long_text: Vec<char> = "MySQL has received positive reviews, and reviewers noticed it performs extremely well in the average case and that the developer interfaces are there, and the documentation (not to mention feedback in the real world via Web sites and the like) is very, very good.[20] It has also been tested to be a fast, stable and true multi-user, multi-threaded SQL database server".chars().collect();
        let mut long_text: Vec<char> = "{'prev_hash':'68186c23c9c65fc40c09c79b0554cd51880f3b23b21cc5b64d1f531d855ddb7a','data':'6'}".chars().collect();

        //println!("{:?}", (91/50) );



        if long_text.len() > 50 {
            let mut pointer = 0;
            let mut writer_pointer = 0;
            loop {
                    if pointer >= (long_text.len() / 50) / 2 {
                        print!("\t    ├──────────│ ");
                    } else {
                        print!("\t    │          │ ");
                    }

                    for i in (pointer * 50)..((pointer * 50) + 50) {

                        if i < long_text.len() {
                            writer_pointer = writer_pointer + 1;
                            print!("{}", long_text[i].green());
                        } else {
                            break;
                        }
                    }
                    if writer_pointer == long_text.len() {
                        break;
                    }
                    //write end
                    print!(" │\n");
                    pointer = pointer + 1;

            }

        } else {
            //write
            print!("\t    ├──────────│ ");
            for i in 0..long_text.len() {
                print!("{}", long_text[i].green());
            }
            //write end
            print!(" │\n");

        }

        // println!("\t    ├──────── {}", "ffffjfjjfjjfjffffjfjjfjjfjffff");
        // println!("\t    │         {}", "ffffjfjjfjjfjffffjfjjfjjfjffff");
        // println!("\t    │         {}", "ffffjfjjfjjfjffffjfjjfjjfjffff");

        println!("\t{}", "fjkgtmfjo".on_blue());

        println!("\t    ▲    ");
        println!("\t    │    ");
        println!("\t    ▼    ");
    }


}

