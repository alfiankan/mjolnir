use crate::Engine;
use owo_colors::OwoColorize;
use regex::Regex;
use std::borrow::Borrow;
use std::io;
use std::io::Write;
use std::process::exit;
use std::env;


fn chained_view(prev_hash: &str, data: &str, hashed_data: &str) {
    // trim every 50 chars
    let long_text: Vec<char> = data.chars().collect();
    println!("\t{}", &prev_hash.on_blue());
    println!("\t\t    │    ");
    println!("\t\t   ─┴─   ");
    if long_text.len() > 50 {
        let mut pointer = 0;
        let mut writer_pointer = 0;
        loop {
            // print!("\t    ├──────────│ ");
            print!("\t");
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
        print!(" \n\t");
        for i in 0..long_text.len() {
            print!("{}", long_text[i].green());
        }
        //write end
    }
    println!("\t\t   ─┬─   ");
    println!("\t\t    │    ");
    println!("\t{}", &hashed_data.on_blue());

    println!("\t\t    ▲    ");
    println!("\t\t    │    ");
    println!("\t\t    ▼    ");
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
    ];
    for help in helps {
        println!();
        print!("\t{}", help[0].on_red());
        print!("{}", " => ");
        print!("{}\t", help[1]);
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

fn eval_command(mql: &str, engine: &Engine) {
    let mql_tokens: Vec<&str> = mql.split(' ').collect();

    match mql_tokens[0].to_uppercase().as_str() {
        "GEN" => match engine.genesis() {
            Ok(key_chain) => {
                println!();
                println!("   <> Generated Key Chain : {}", key_chain.green());
                println!("   <> {} : Now you can insert new box", "Hint".on_red());
                println!(
                    "   <> {} : INSERT TO {} 'mystringdata' ",
                    "Example".on_red(),
                    key_chain.green()
                );
            }
            Err(err) => {
                println!("\t<> Error : {}", err.red());
            }
        },
        "RECORDS" => {
            match engine.list_records() {
                Ok(records) => {
                    println!();
                    println!("   <> All Records :");
                    println!("   ================");
                    let mut numbering = 1;
                    for x in records {
                        println!("   [{}]\t {}", numbering.yellow(), x[0]);
                        println!();
                        numbering = numbering + 1;
                    }
                    println!("   ============================================================ Found : {:?}", numbering - 1);
                }
                Err(err) => {
                    println!("\t<> Error : {}", err.red());
                }
            }
        }
        "INSERT" => {
            if mql_tokens.len() < 4 {
                println!(
                    "{}",
                    "invalid mql should INSERT TO :chain_key :string_data".red()
                )
            } else {
                if mql_tokens[1].to_uppercase().as_str() == "TO" {
                    let payload_indexs = Regex::new(r"'(.+?)'").unwrap().find(&mql).unwrap();

                    match engine.insert_to_chain(
                        (&mql[payload_indexs.start()..payload_indexs.end()]).to_string(),
                        mql_tokens[2],
                    ) {
                        Ok(success) => {
                            println!();
                            println!(
                                "   <>\tBox inserted to record with key chain {} ",
                                mql_tokens[2]
                            );
                            println!("   [{}]\tprev \t: {}", "*".green(), success[0].green());
                            println!("   [{}]\tdata \t: {}", "*".green(), success[1].green());
                            println!("   [{}]\thash \t: {}", "*".green(), success[2].green());
                            println!();
                        }
                        Err(_) => {}
                    }
                } else {
                    // unprocessable
                    println!(
                        "{}",
                        "invalid mql should INSERT TO :chain_key :string_data".red()
                    )
                }
            }
        }
        "SELECT" => {
            if mql_tokens.len() < 2 {
                println!("{}", "invalid mql see help".red())
            } else {
                println!();
                // if selecet all
                if mql_tokens.len() == 3 {
                    match engine.get_all_box_from_record(mql_tokens[1].to_string()) {
                        Ok(result) => {
                            for x in result.boxs {
                                chained_view(
                                    x.prev_hash.to_string().as_str(),
                                    x.data.to_string().as_str(),
                                    x.hashed_data.to_string().as_str(),
                                );
                            }
                            println!(
                                "========== {} valid from {} box in chain ==========",
                                result.total_valid_data, result.total_data
                            );
                        }
                        Err(err) => {
                            println!("\t<> Error : {}", err.red());
                        }
                    }
                } else if mql_tokens.len() == 2 {
                    match engine.find_last_box(mql_tokens[1].to_string()) {
                        Ok(result) => {
                            for x in result.boxs {
                                chained_view(
                                    x.prev_hash.to_string().as_str(),
                                    x.data.to_string().as_str(),
                                    x.hashed_data.to_string().as_str(),
                                );
                            }
                        }
                        Err(err) => {
                            println!("\t<> Error : {}", err.red());
                        }
                    }
                }
            }
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

pub fn start_cli() {

    //read env
    match env::var("MJL_DATA_STORE") {
        Ok(file_name) => {

            let engine = Engine::new(file_name.to_string().as_str());
            let mut mql = String::new();

            let input = io::stdin();
            let mut lines = 1;
            loop {
                mql.clear();
                println!();
                print!("[{}] [mql] > ", lines.yellow());
                io::stdout().flush().expect("CLI Fatal Error");

                match input.read_line(&mut mql) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error => {}", err);
                        continue;
                    }
                }
                eval_command(mql.trim(), engine.borrow());
                lines = lines + 1;
            }
        }
        Err(_) => {
            println!("env MJL_DATA_STORE ot set");
        }
    }

}
