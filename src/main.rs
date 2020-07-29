use std::io;

// get number out of a chunk as if it had length of 3
fn to_do_give_this_function_cool_and_descriptive_name(chunk: &[u8], index_to_get: usize) -> usize {
    (chunk[index_to_get - (3 - chunk.len())] - 48) as usize
}

fn chunk_to_words(chunk: &[u8], dict: &Vec<Vec<Vec<&str>>>) -> String {
    let mut result = String::new();

    let mut special_case: usize;
    let mut units: usize;

    // the foor loop acts as if the chunk was shifted to the right
    for i in (3 - chunk.len())..3 {
        if to_do_give_this_function_cool_and_descriptive_name(&chunk, i) == 0 {
            continue;
        } else if i == 1 && to_do_give_this_function_cool_and_descriptive_name(&chunk, 1) == 1 {
            special_case = 1;
            units = to_do_give_this_function_cool_and_descriptive_name(&chunk, 2);
            if units == 0 {
                special_case = 0;
            } else {
                units -= 1;
            }
        } else {
            special_case = 0;
            units = to_do_give_this_function_cool_and_descriptive_name(&chunk, i) - 1;
        }

        result.push_str(dict[2 - i][special_case][units]);
        if (i == 1 && special_case == 1) || i == 2 {
            break;
        };
        result.push_str(" ");
    }

    result.trim().to_string()
}
fn main() {
    let dict_pl = vec![
        vec![vec![
            "jeden",
            "dwa",
            "trzy",
            "cztery",
            "pięć",
            "sześć",
            "siedem",
            "osiem",
            "dziewięć",
        ]],
        vec![
            vec![
                "dziesięć",
                "dwadzieścia",
                "trzydzieści",
                "czterdzieści",
                "pięćdziesiąt",
                "sześćdziesiąt",
                "siedemdziesiąt",
                "osiemdziesiąt",
                "dziewięćdziesiąt",
            ],
            vec![
                "jedenaście",
                "dwanaście",
                "trzynaście",
                "czternaście",
                "piętnaście",
                "szesnaście",
                "siedemnaście",
                "osiemnaście",
                "dziewiętnaście",
            ],
        ],
        vec![vec![
            "sto",
            "dwieście",
            "trzysta",
            "czterysta",
            "pięćset",
            "sześćset",
            "siedemset",
            "osiemset",
            "dziewięćset",
        ]],
    ];
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: String = input
        .trim()
        .parse::<i128>()
        .expect("That doesn't look like a valid integer!")
        .to_string();

    for chunk in num.as_bytes().chunks(3) {
        println!("chunk: {:#?}", chunk);
        println!("word: {:#?}", chunk_to_words(chunk, &dict_pl));
    }
}
