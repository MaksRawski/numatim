extern crate numatim;
use numatim::*;

use clap::{App, Arg};
use dirs::config_dir;
use regex::Regex;
use std::fs;
use std::io;
use std::process;

fn main() {
    let matches = App::new("numatim")
        .version("0.2")
        .author("Maks Rawski <maksymilian.rawski@tutanota.com>")
        .about("Returns a number given from STDIN in words")
        .arg(
            Arg::with_name("language")
                .short("l")
                .long("language")
                .value_name("language")
                .help("Sets a language to use from ~/.config/numatim")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .index(1)
                .help("Number to use, will be taken from STDIN if not provided"),
        )
        .get_matches();

    let lang = matches.value_of("language").unwrap_or("en");
    let mut dict_location = config_dir().unwrap();
    dict_location.push(format!("numatim/dicts/{}.json", lang));

    let dict = match fs::read_to_string(&dict_location) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "Failed to read \"{}\", {}",
                dict_location.to_str().unwrap(),
                e
            );
            process::exit(1);
        }
    };
    let dict: Dict = match serde_json::from_str(&dict) {
        Ok(json) => json,
        Err(e) => {
            eprintln!(
                "Failed to parse \"{}\", {}",
                dict_location.to_str().unwrap(),
                e
            );
            process::exit(1);
        }
    };
    let mut input = String::new();

    match matches.value_of("INPUT") {
        Some(v) => input = v.to_string(),
        None => {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }
    }

    // remove everything that is not a digit from input
    let re = Regex::new(r"[^\d]").unwrap();
    input = re.replace_all(&input, "").to_owned().to_string();

    match verbatim(input, dict) {
        Ok(v) => println!("{}", v),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}
