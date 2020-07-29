use regex::Regex;
use serde_json;
use std::fs;
use std::io;
use std::process;

fn get_usize_out_of_shifted_right_chunk(chunk: &[u8], index_to_get: usize) -> usize {
    (chunk[index_to_get - (3 - chunk.len())] - 48) as usize
}

fn chunk_to_words(chunk: &[u8], dict: &Vec<Vec<Vec<&str>>>) -> String {
    let mut result = String::new();

    let mut special_case: usize;
    let mut units: usize;

    // the for loop acts as if the chunk was shifted to the right
    for i in (3 - chunk.len())..3 {
        if get_usize_out_of_shifted_right_chunk(&chunk, i) == 0 {
            continue;
        } else if i == 1 && get_usize_out_of_shifted_right_chunk(&chunk, 1) == 1 {
            special_case = 1;
            units = get_usize_out_of_shifted_right_chunk(&chunk, 2);
            if units == 0 {
                special_case = 0;
            } else {
                units -= 1;
            }
        } else {
            special_case = 0;
            units = get_usize_out_of_shifted_right_chunk(&chunk, i) - 1;
        }

        result.push_str(dict[2 - i][special_case][units]);
        if (i == 1 && special_case == 1) || i == 2 {
            break;
        };
        result.push_str(" ");
    }

    result.trim().to_string()
}

fn power_of_hundred<'a>(index: usize, multiple: &[u8], powers: &'a Vec<&str>) -> String {
    match index {
        0 => format!(""),
        1 => match multiple {
            [hundreds, tens, units] if *units == b'0' && *tens == b'0' && *hundreds == b'0' => {
                format!("")
            }
            [_, tens, units] if *units == b'1' && *tens == b'0' => format!(" tysiąc "),
            [_, tens, units] if *units > b'1' && *units < b'5' && *tens != b'1' => {
                format!(" tysiące ")
            }
            _ => format!(" tysięcy "),
        },
        _ => match multiple {
            [hundreds, tens, units] if *units == b'0' && *tens == b'0' && *hundreds == b'0' => {
                format!("")
            }
            [_, tens, units] if *units == b'1' && *tens == b'0' => {
                format!(" {} ", powers[index - 2])
            }
            [_, tens, units] if *units > b'1' && *units < b'5' && *tens != b'1' => {
                format!(" {}y ", powers[index - 2])
            }
            _ => format!(" {}ów ", powers[index - 2]),
        },
    }
}

fn main() {
    let dict_name = "dict_pl.json";
    let powers_name = "powers.json";

    let dict = match fs::read_to_string(dict_name) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read \"{}\", {}", dict_name, e);
            process::exit(1);
        }
    };
    let dict: Vec<Vec<Vec<&str>>> = match serde_json::from_str(&dict) {
        Ok(json) => json,
        Err(e) => {
            println!("Failed to parse \"{}\", {}", dict_name, e);
            process::exit(1);
        }
    };

    let powers = match fs::read_to_string(powers_name) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read \"{}\", {}", powers_name, e);
            process::exit(1);
        }
    };
    let powers: Vec<&str> = match serde_json::from_str(&powers) {
        Ok(json) => json,
        Err(e) => {
            println!("Failed to parse \"{}\", {}", powers_name, e);
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

    for chunk in input.as_bytes().chunks(3) {
        hundreds -= 1;
        print!("{}", chunk_to_words(chunk, &dict));
        print!("{}", power_of_hundred(hundreds, &chunk, &powers));
    }
    println!();
}
