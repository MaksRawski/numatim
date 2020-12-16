extern crate numatim;
use numatim::*;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    fn setup(lang: String) {
        let dict_location = format!("dicts/{}.json", lang);
        let dict = fs::read_to_string(dict_location).unwrap();
        let dict: Vec<Vec<Vec<&str>>> = serde_json::from_str(&dict).unwrap();
    }

    #[test]
    fn test_get_usize_out_of_shifted_right_chunk() {
        assert_eq!(3, 3);
    }
}
