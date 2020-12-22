use serde::{Deserialize, Serialize};
use serde_json::Value;

// Number needs to be sanitized before usage.
pub fn verbatim(number: String, dict: Dict) -> Result<String, String> {
    let mut number = number;
    // pad with leading zeros if input is not divisible by 3
    if number.len() % 3 != 0 {
        let leading_zeros = "0".repeat(3 - (number.len() % 3));
        number.insert_str(0, &leading_zeros);
    }
    let mut res = String::new();

    let mut hundreds: usize = number.len() / 3;
    if hundreds > dict.powers.len() {
        return Err("Too long number".to_string());
    }
    for chunk in number.chars().collect::<Vec<char>>().chunks(3) {
        hundreds -= 1;
        let chunk_str: String = chunk.iter().collect();
        let chunk_num = chunk_str.parse().unwrap();

        res = format!("{}{}", res, chunk_to_words(chunk_num, &dict.numbers));
        res = format!(
            "{}{} ",
            res,
            power_of_hundred(hundreds, chunk_num, &dict.powers)
        );
    }
    Ok(res)
}

// TODO: refactor this and maybe even make it a method on top of Dict
pub fn chunk_to_words(chunk: u16, numbers: &Vec<Value>) -> String {
    let mut result = String::new();
    let mut nums = Vec::new();

    nums.push(numbers[0].as_array().unwrap());
    nums.push(numbers[1].as_array().unwrap());
    nums.push(numbers[2].as_array().unwrap());

    if chunk == 0 {
        return result;
    } else {
        // 100, 200, ..., 900
        if chunk >= 100 {
            let number = nums[2][(chunk / 100 - 1) as usize].as_str().unwrap();
            result = format!("{} ", &number[..]);
        }

        // 11-19
        if (11..=19).contains(&(chunk % 100)) {
            // this will only work if language has hardcoded tenths
            let number = &nums[1][(chunk % 100 - 11) as usize].as_array().unwrap()[0]
                .as_str()
                .unwrap();
            result = format!("{}{} ", result, number)
        }
        // 10, 20, ..., 100
        else if chunk % 100 >= 20 || chunk % 100 == 10 {
            let number = &mut nums[1][(chunk % 100 / 10 - 1) as usize].as_array().unwrap()[1]
                .as_str()
                .unwrap()
                .to_string();
            let deli = number.chars().last().unwrap();

            if deli == '-' && chunk % 10 == 0 {
                number.truncate(number.len() - 1);
                result = format!("{}{} ", result, number)
            } else if deli != '-' {
                result = format!("{}{} ", result, number)
            } else {
                result = format!("{}{}", result, number)
            }
        }

        // 1-9
        if chunk % 10 != 0 && (chunk % 100 / 10) != 1 {
            let digit = nums[0][(chunk % 10 - 1) as usize].as_str().unwrap();
            result = format!("{}{} ", result, digit)
        }
    }
    result
}

// TODO: refactor this and maybe even make it a method on top of Dict
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
                res = format!("{}{}", res, delimiter);
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
