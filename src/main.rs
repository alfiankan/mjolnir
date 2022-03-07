use std::io;
use std::io::Write;
use std::process::exit;
use owo_colors::OwoColorize;
use regex::Regex;


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

    // (["'])(.*?[^\\])\1
    let re = Regex::new(r"'(.+?)'").unwrap();


    if mql_tokens.len() < 4 {
            println!("{}","invalid mql should INSERT TO :chain_key :string_data".red())

    } else {
        if mql_tokens[1].to_uppercase().as_str() == "TO" {
            println!("INSERTING TO CHAIN {}", mql_tokens[2].blue());

        } else {
            // unprocessable
            println!("{}","invalid mql should INSERT TO :chain_key :string_data".red())
        }

    }

}

fn eval_command(mql: &str) {
    let mql_tokens: Vec<&str> = mql.split( ' ').collect();

    match mql_tokens[0].to_uppercase().as_str() {
        "GENESIS" => {
            println!("GENERATE NEW CHAIN");
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

    loop {
        mql.clear();
        print!("[mql] > ");
        io::stdout().flush().expect("Fatal Error");

        match input.read_line(&mut mql) {
            Ok(_) => {}
            Err(err) => {
                println!("Error => {}", err);
                continue;
            }
        }
        eval_command(mql.trim());
    }


}
