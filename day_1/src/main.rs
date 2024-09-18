use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Lines},
};

use helpers::{read_lines, trie::Trie};

fn day_1_1(lines_buffer: Lines<BufReader<File>>) -> Result<i32, io::Error> {
    let mut total = 0;

    for line in lines_buffer {
        let mut first: Option<String> = None;
        let mut last: Option<String> = None;

        for char in line?.chars() {
            if char.is_ascii_digit() {
                if first.is_none() {
                    first = Some(char.to_string());
                } else {
                    last = Some(char.to_string());
                }
            }
        }

        let num_str = match (first, last) {
            (Some(first), Some(last)) => format!("{}{}", first, last),
            (Some(first), None) => format!("{}{}", first, first),
            (first, last) => {
                eprintln!("Unexpected values: {:?} {:?}", first, last);
                "0".to_owned()
            }
        };

        total += match num_str.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Failed to parse value: {}", num_str);
                0
            }
        };
    }

    Ok(total)
}

fn day_1_2(lines_buffer: Lines<BufReader<File>>) -> Result<i32, io::Error> {
    let str_num_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut trie = Trie::new();
    for key in str_num_map.keys() {
        trie.insert(key);
    }
    let mut total = 0;

    for line in lines_buffer {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        let line_length = chars.len();

        let mut first: Option<String> = None;
        let mut last: Option<String> = None;
        let mut window_left: usize = 0;
        let mut window_right: usize = 0;
        let mut current_node = trie.get_root();

        while window_left < line_length {
            let char = chars[window_right];

            // If is a digit, just take it. Don't bother with the trie.
            if char.is_ascii_digit() {
                if first.is_none() {
                    first = Some(char.to_string());
                } else {
                    last = Some(char.to_string());
                }

                window_left += 1;
                window_right = window_left;
                current_node = trie.get_root();
                continue;
            }

            let new_node = current_node.get_child(char);
            match new_node {
                Some(new_node) => {
                    if new_node.is_end() {
                        let str_num = &line[window_left..window_right + 1];

                        // Total control over Trie and HashMap, if exists in
                        // one of them, it will exist in the other.
                        let num = str_num_map.get(str_num).unwrap().to_string();
                        if first.is_none() {
                            first = Some(num);
                        } else {
                            last = Some(num);
                        }

                        // Found: increase left window position and reset trie
                        // and right window position.
                        window_left += 1;
                        window_right = window_left;
                        current_node = trie.get_root();
                    } else {
                        // Partially found: increase right window position.
                        window_right += 1;
                        // But only if is still in the string, if not,
                        // increase left window position and reset trie.
                        if window_right >= line_length {
                            window_left += 1;
                            window_right = window_left;
                            current_node = trie.get_root();
                        } else {
                            current_node = new_node;
                        }
                    }
                }
                None => {
                    // Not found: same as found. Increase position and reset
                    // trie and right window position.
                    window_left += 1;
                    window_right = window_left;
                    current_node = trie.get_root();
                }
            };
        }

        let num_str = match (first, last) {
            (Some(first), Some(last)) => format!("{}{}", first, last),
            (Some(first), None) => format!("{}{}", first, first),
            _ => "0".to_owned(),
        };

        total += match num_str.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Failed to parse value: {}", num_str);
                0
            }
        };
    }

    Ok(total)
}

fn main() -> Result<(), io::Error> {
    let path = "day_1/data/input.txt";

    let day_1_1_result = day_1_1(read_lines(path)?)?;
    println!("Day 1 Part 1 Result: {}", day_1_1_result);

    let day_1_2_result = day_1_2(read_lines(path)?)?;
    println!("Day 1 Part 2 Result: {}", day_1_2_result);

    Ok(())
}
