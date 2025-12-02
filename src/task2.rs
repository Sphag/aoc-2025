use std::fs::read_to_string;
use std::collections::HashSet;

use itertools::Itertools;


fn part1(id_ranges: Vec<(&str, &str)>) -> u64 {
    let mut invalid_ids: HashSet<u64> = HashSet::new();
    for id_range in id_ranges {
        let id_range_start: u64 = id_range.0.parse().unwrap();
        let id_range_end: u64 = id_range.1.parse().unwrap();

        for id in id_range_start .. id_range_end + 1 {
            let id_chars = id.to_string().into_bytes();
            if id_chars.len() % 2 == 1 {
                continue;
            }

            let mut is_valid_id = false;
            for i in 0 .. id_chars.len() / 2 {
                if id_chars[i] != id_chars[(id_chars.len() + 1) / 2 + i] {
                    is_valid_id = true;
                }
            }

            if !is_valid_id {
                invalid_ids.insert(id);
            }
        }
    }

    return invalid_ids.iter().fold(0, |acc, &e| acc + e);
}


pub fn execute_part1_from_input(input: &str) {
    let file_content = read_to_string(input).expect("Couldn't open file");
    let id_ranges: Vec<(&str, &str)> =file_content
            .split(",")
            .map(|x| x.split("-").collect_tuple().unwrap())
            .collect();
    println!("task2::part1 - result is {}", part1(id_ranges));
}


fn part2(id_ranges: Vec<(&str, &str)>) -> u64 {
    let mut invalid_ids: HashSet<u64> = HashSet::new();
    for id_range in id_ranges {
        let id_range_start: u64 = id_range.0.parse().unwrap();
        let id_range_end: u64 = id_range.1.parse().unwrap();

        for id in id_range_start .. id_range_end + 1 {
            let id_chars = id.to_string().into_bytes();
            for len_repeated in 1 .. id_chars.len() / 2 + 1{
                if id_chars.len() % len_repeated != 0 {
                    continue;
                }
                
                let mut is_valid_id = false;
                for current_digit_index in len_repeated .. id_chars.len() {
                    if id_chars[current_digit_index % len_repeated] != id_chars[current_digit_index] {
                        is_valid_id = true;
                    }
                }

                if !is_valid_id {
                    invalid_ids.insert(id);
                }
            }

        }
    }

    return invalid_ids.iter().fold(0, |acc, &e| acc + e);
}


pub fn execute_part2_from_input(input: &str) {
    let file_content = read_to_string(input).expect("Couldn't open file");
    let id_ranges: Vec<(&str, &str)> =file_content
            .split(",")
            .map(|x| x.split("-").collect_tuple().unwrap())
            .collect();
    println!("task2::part1 - result is {}", part2(id_ranges));
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_cases() {
        let id_ranges: Vec<(&str, &str)> = vec![
            ("11",         "22"),
            ("95",         "115"),
            ("998",        "1012"),
            ("1188511880", "1188511890"),
            ("222220",     "222224"),
            ("1698522",    "1698528"),
            ("446443",     "446449"),
            ("38593856",   "38593862"),
        ];

        assert_eq!(part1(id_ranges), 1227775554);
    }

    #[test]
    fn test_part2_cases() {
        let id_ranges: Vec<(&str, &str)> = vec![
            ("11",         "22"),
            ("95",         "115"),
            ("998",        "1012"),
            ("1188511880", "1188511890"),
            ("222220",     "222224"),
            ("1698522",    "1698528"),
            ("446443",     "446449"),
            ("38593856",   "38593862"),
            ("565653",     "565659"),
            ("824824821",  "824824827"),
            ("2121212118", "2121212124"),
        ];

        assert_eq!(part2(id_ranges), 4174379265);
    }
}