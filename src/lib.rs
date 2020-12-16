use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn chunk_to_words(chunk: u16, numbers: &Vec<Value>) -> String {
    let mut result = String::new();
    if chunk == 0 {
        return result;
    } else {
        if chunk >= 100 {
            let number = numbers[2].as_array().unwrap()[(chunk / 100 - 1) as usize]
                .as_str()
                .unwrap();
            result = format!("{} ", &number[..]);
        } else {
            return "".to_string();
        }
    }
    // result.push_str(numbers[2][(chunk / 100) as usize].as_str().unwrap());
    // result.push_str(numbers[0][(chunk % 10) as usize].as_str().unwrap());
    // result.push_str(numbers[1][(chunk % 10 / 10) as usize][1].as_str().unwrap());
    result
}

// TODO: refactor this
pub fn power_of_hundred<'a>(index: usize, multiple: u16, powers: &Vec<Value>) -> String {
    let mut res = String::new();

    if index != 0 {
        match powers[index].as_array() {
            // if all the forms of a word are provided
            Some(v) => {
                if multiple == 0 {
                    return "".to_string();
                } else if multiple == 1 {
                    res = v[0].as_str().unwrap().to_string();
                } else if (10..=21).contains(&multiple) {
                    res = v[2].as_str().unwrap().to_string();
                } else {
                    res = match multiple % 10 {
                        1 => v[2].as_str().unwrap().to_string(),
                        2..=4 => v[1].as_str().unwrap().to_string(),
                        5..=9 => v[2].as_str().unwrap().to_string(),
                        _ => v[2].as_str().unwrap().to_string(),
                    };
                }
            }
            // if powers[index] is not array - single word
            None => {
                res = powers[index].as_str().unwrap().to_string();
                let delimiters = powers[0].as_array().unwrap();
                let delimiter: &str;

                if multiple == 0 {
                    return "".to_string();
                } else if multiple == 1 {
                    delimiter = delimiters[0].as_str().unwrap();
                } else if (10..=21).contains(&multiple) {
                    delimiter = delimiters[2].as_str().unwrap();
                } else {
                    delimiter = match multiple % 10 {
                        1 => delimiters[2].as_str().unwrap(),
                        2..=4 => delimiters[1].as_str().unwrap(),
                        5..=9 => delimiters[2].as_str().unwrap(),
                        _ => delimiters[2].as_str().unwrap(),
                    };
                }
                res = format!("{}{} ", res, delimiter);
            }
        };
    }
    res
}

#[derive(Serialize, Deserialize)]
pub struct Dict {
    pub numbers: Vec<Value>,
    pub powers: Vec<Value>,
}
