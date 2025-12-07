use std::fs::read_to_string;
use std::collections::{HashSet, HashMap};


fn part1(manifold: Vec<Vec<char>>) -> u64 {
    let mut beam_split_count: u64 = 0;
    let mut beam_indices: HashSet<usize> = HashSet::new();
    for row in manifold {
        for (element_index, element) in row.iter().enumerate() {
            match element {
                '.' => {},
                'S' => { beam_indices.insert(element_index); },
                '^' => {
                    if beam_indices.contains(&element_index) {
                        beam_split_count += 1;

                        beam_indices.remove(&element_index);
                        if element_index > 0 {
                            beam_indices.insert(element_index - 1);
                        }
                        if element_index < row.len() - 1 {
                            beam_indices.insert(element_index + 1);
                        }
                    }
                },
                _ => assert!(false, "Found element that should not be there {element}")
            };
        }
    }
    beam_split_count
}


pub fn execute_part1_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    println!("task7::part1 - result is {}", part1(file_input.lines().map(|x| x.chars().collect()).collect()));
}


fn part2(manifold: Vec<Vec<char>>) -> u64 {
    let mut beam_indices_to_timeline_count: HashMap<usize, u64> = HashMap::new();
    for row in manifold {
        for (element_index, element) in row.iter().enumerate() {
            match element {
                '.' => {},
                'S' => { beam_indices_to_timeline_count.insert(element_index, 1); },
                '^' => {
                    if let Some(&timeline_count) = beam_indices_to_timeline_count.get(&element_index) {
                        if element_index > 0 {
                            *beam_indices_to_timeline_count.entry(element_index - 1).or_insert(0) += timeline_count;
                        }
                        if element_index < row.len() - 1 {
                            *beam_indices_to_timeline_count.entry(element_index + 1).or_insert(0) += timeline_count;
                        }
                        beam_indices_to_timeline_count.remove(&element_index);
                    }
                },
                _ => assert!(false, "Found element that should not be there {element}")
            };
        }
    }
    beam_indices_to_timeline_count.iter().fold(0_u64, |acc, (_, &timeline_count)| acc + timeline_count)
}


pub fn execute_part2_from_input(input: &str) {
let file_input = read_to_string(input).expect("Couldn't read file");
    println!("task7::part2 - result is {}", part2(file_input.lines().map(|x| x.chars().collect()).collect()));
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_cases() {
        let manifold: Vec<Vec<char>> = vec![
            ".......S.......".chars().collect(),
            "...............".chars().collect(),
            ".......^.......".chars().collect(),
            "...............".chars().collect(),
            "......^.^......".chars().collect(),
            "...............".chars().collect(),
            ".....^.^.^.....".chars().collect(),
            "...............".chars().collect(),
            "....^.^...^....".chars().collect(),
            "...............".chars().collect(),
            "...^.^...^.^...".chars().collect(),
            "...............".chars().collect(),
            "..^...^.....^..".chars().collect(),
            "...............".chars().collect(),
            ".^.^.^.^.^...^.".chars().collect(),
            "...............".chars().collect(),
        ];

        assert_eq!(part1(manifold), 21);
    }

    #[test]
    fn test_part2_cases() {
        let manifold: Vec<Vec<char>> = vec![
            ".......S.......".chars().collect(),
            "...............".chars().collect(),
            ".......^.......".chars().collect(),
            "...............".chars().collect(),
            "......^.^......".chars().collect(),
            "...............".chars().collect(),
            ".....^.^.^.....".chars().collect(),
            "...............".chars().collect(),
            "....^.^...^....".chars().collect(),
            "...............".chars().collect(),
            "...^.^...^.^...".chars().collect(),
            "...............".chars().collect(),
            "..^...^.....^..".chars().collect(),
            "...............".chars().collect(),
            ".^.^.^.^.^...^.".chars().collect(),
            "...............".chars().collect(),
        ];

        assert_eq!(part2(manifold), 40);
    }
}