
use database::database::JaySonDB;
use std::{env, io::BufRead, usize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath;
    if args.len() < 2 {
        filepath = None;
    } else {
        filepath = Some(args[1].clone());
    }

    let mut db = JaySonDB::<String, String>::new(filepath);

    println!("CLI ready");
    let input = std::io::stdin();
    let lines = input.lock().lines();
    for line in lines {
        match line {
            Ok(read_line) => {
                parse(&mut db, read_line);
            }
            Err(e) => println!("Couldn't read line. Quitting ..."),
        }
    }
}

fn parse(db: &mut JaySonDB<String, String>, line: String) {
    let test_line = line.clone();
    let split_line: Vec<&str> = line.split_whitespace().collect();
    match split_line[0] {
        "insert" => {
            if split_line.len() - 1 != 2 {
                println!("insert requires 2 parameters: Key and Value");
                return;
            }

            db.insert(split_line[1].to_string(), split_line[2].to_string());
        }
        "delete" => {
            if split_line.len() - 1 != 1 {
                println!("delete requires 1 parameter: Key");
                return;
            }

            db.delete(split_line[1].to_string());
        }
        "lookup" => {
            if split_line.len() - 1 != 1 {
                println!("lookup requires 1 parameter: Key");
                return;
            }
            db.lookup(split_line[1].to_string());
        }
        "list" => {
            let all_entries = db.list_all();
            for el in all_entries {
                match el {
                    Some(entry) => {
                        println!("Key: {} | Value: {}", entry.key, entry.value);
                    }
                    None => {}
                }
            }
        }
        "logic" => unimplemented!(),
        _ => println!("No such command"),
    }
}
