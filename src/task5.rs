use std::fs::read_to_string;

use itertools::Itertools;


fn part1(fresh_id_ranges: Vec<(u64, u64)>, available_ingredients: Vec<u64>) -> u64 {
    let mut fresh_count: u64 = 0;
    for ingredient in available_ingredients {
        for (fresh_id_range_start, fresh_id_range_end) in &fresh_id_ranges {
            if ingredient >= *fresh_id_range_start && ingredient <= *fresh_id_range_end {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}


pub fn execute_part1_from_input(input: &str) {
    let mut fresh_id_ranges: Vec<(u64, u64)> = Vec::new();
    let mut available_ingredients: Vec<u64> = Vec::new();
    let file_input = read_to_string(input).expect("Couldn't read file");
    let mut is_parsing_ranges = true;
    for line in file_input.lines() {
        if line.is_empty() {
            is_parsing_ranges = false;
            continue;
        }

        if is_parsing_ranges {
            fresh_id_ranges.push(line.split('-').map(|x| x.parse().unwrap()).collect_tuple().unwrap());
        } else {
            available_ingredients.push(line.parse().unwrap());
        }
    }
    println!("task5::part1 - result is {}", part1(fresh_id_ranges, available_ingredients));
}


fn part2(fresh_id_ranges: Vec<(u64, u64)>) -> u64 {
    let mut fresh_id_ranges_merged: Vec<(u64, u64)> = Vec::new();

    for (fresh_id_range_start, fresh_id_range_end) in fresh_id_ranges.iter().sorted() {
        let mut is_range_merged = false;
        for current_merged_range_idx in 0 .. fresh_id_ranges_merged.len() {
            let merged_range_len = fresh_id_ranges_merged[current_merged_range_idx].1.max(*fresh_id_range_end) - 
                                        fresh_id_ranges_merged[current_merged_range_idx].0.min(*fresh_id_range_start);
            let current_range_len = fresh_id_ranges_merged[current_merged_range_idx].1 -
                                         fresh_id_ranges_merged[current_merged_range_idx].0;
            let range_len = fresh_id_range_end - fresh_id_range_start;
            if merged_range_len <= current_range_len + range_len {
                is_range_merged = true;
                fresh_id_ranges_merged[current_merged_range_idx].0 = fresh_id_ranges_merged[current_merged_range_idx].0.min(*fresh_id_range_start);
                fresh_id_ranges_merged[current_merged_range_idx].1 = fresh_id_ranges_merged[current_merged_range_idx].1.max(*fresh_id_range_end);
                break;
            }
        }

        if !is_range_merged {
            fresh_id_ranges_merged.push((*fresh_id_range_start, *fresh_id_range_end));
        }
    }

    let mut fresh_id_count: u64 = 0;
    for (range_start, range_end) in fresh_id_ranges_merged {
        fresh_id_count += range_end - range_start + 1;
    }

    fresh_id_count
}


pub fn execute_part2_from_input(input: &str) {
    let mut fresh_id_ranges: Vec<(u64, u64)> = Vec::new();
    let file_input = read_to_string(input).expect("Couldn't read file");
    for line in file_input.lines() {
        if line.is_empty() {
            break;
        }

        fresh_id_ranges.push(line.split('-').map(|x| x.parse().unwrap()).collect_tuple().unwrap());
    }
    println!("task5::part2 - result is {}", part2(fresh_id_ranges));
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_cases() {
        let fresh_id_ranges: Vec<(u64, u64)> = vec![
            (3,  5 ),
            (10, 14),
            (16, 20),
            (12, 18),
        ];
        let available_ingridients: Vec<u64> = vec![
            1,
            5,
            8,
            11,
            17,
            32,
        ];

        assert_eq!(part1(fresh_id_ranges, available_ingridients), 3);
    }

    #[test]
    fn test_part2_cases() {
        let fresh_id_ranges: Vec<(u64, u64)> = vec![
            (3,  5 ),
            (10, 14),
            (16, 20),
            (12, 18),
        ];

        assert_eq!(part2(fresh_id_ranges), 14);
    }
}