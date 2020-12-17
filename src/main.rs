extern crate numatim;
use numatim::*;

use clap::{App, Arg};

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
            Arg::with_name("dictionary")
                .short("d")
                .long("dictionary")
                .value_name("dictionary")
                .help("Location of the dictionary to use")
                .takes_value(true),
        )
        .get_matches();

    let lang = matches.value_of("lang").unwrap_or("pl");
    let dict_location = format!("dicts/{}.json", lang);

    let dict = match fs::read_to_string(&dict_location) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read \"{}\", {}", dict_location, e);
            process::exit(1);
        }
    };
    let dict: Dict = match serde_json::from_str(&dict) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to parse \"{}\", {}", dict_location, e);
            process::exit(1);
        }
    };
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // remove everything that is not a digit from input
    let re = Regex::new(r"[^\d]").unwrap();
    input = re.replace_all(&input, "").to_owned().to_string();

    // pad with leading zeros if input is not divisible by 3
    if input.len() % 3 != 0 {
        let leading_zeros = "0".repeat(3 - (input.len() % 3));
        input.insert_str(0, &leading_zeros);
    }

    let mut hundreds: usize = input.len() / 3;
    if hundreds >= dict.powers.len() + 3 {
        eprintln!("Too long input");
        process::exit(1);
    }
    for chunk in input.chars().collect::<Vec<char>>().chunks(3) {
        hundreds -= 1;
        let chunk_str: String = chunk.iter().collect();
        let chunk_num = chunk_str.parse().unwrap();

        print!("{}", chunk_to_words(chunk_num, &dict.numbers));
        print!("{} ", power_of_hundred(hundreds, chunk_num, &dict.powers));
    }
    println!();
}
