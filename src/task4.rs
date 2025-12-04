use std::fs::read_to_string;


fn part1(grid: Vec<Vec<char>>) -> u64 {
    let mut accessable_rolls: u64 = 0;
    for i in 0 .. grid.len() {
        for j in 0 .. grid[i].len() {
            if grid[i][j] == '.' {
                continue;
            }

            let mut adj_roll_count: u64 = 0;
            for ri in i.saturating_sub(1) .. grid.len().min(i+2) {
                for rj in j.saturating_sub(1) .. grid[i].len().min(j+2) {
                    if grid[ri][rj] == '@' {
                        adj_roll_count += 1;
                    }
                }
            }

            if adj_roll_count <= 4 {
                accessable_rolls += 1;
            }
        }
    }

    accessable_rolls
}


pub fn execute_part1_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let grid: Vec<Vec<char>> = file_input.lines().map(|e| e.chars().collect()).collect();
    println!("task4::part1 - result is {}", part1(grid));
}


fn part2(mut grid: Vec<Vec<char>>) -> u64 {
    let mut accessable_rolls: u64 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < grid.len() {
        while j < grid[i].len() {
            if grid[i][j] == '.' {
                j += 1;
                continue;
            } 

            let mut adj_roll_count: u64 = 0;
            for ri in i.saturating_sub(1) .. grid.len().min(i+2) {
                for rj in j.saturating_sub(1) .. grid[i].len().min(j+2) {
                    if grid[ri][rj] == '@' {
                        adj_roll_count += 1;
                    }
                }
            }

            if adj_roll_count <= 4 {
                accessable_rolls += 1;
                grid[i][j] = '.';
                i = i.saturating_sub(1);
                j = j.saturating_sub(1);
            } else {
                j += 1;
            }
        }

        i += 1;
        j = 0;
    }

    accessable_rolls
}


pub fn execute_part2_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let grid: Vec<Vec<char>> = file_input.lines().map(|e| e.chars().collect()).collect();
    println!("task4::part2 - result is {}", part2(grid));
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_cases() {
        let grid: Vec<Vec<char>> = vec![
            "..@@.@@@@.".chars().collect(),
            "@@@.@.@.@@".chars().collect(),
            "@@@@@.@.@@".chars().collect(),
            "@.@@@@..@.".chars().collect(),
            "@@.@@@@.@@".chars().collect(),
            ".@@@@@@@.@".chars().collect(),
            ".@.@.@.@@@".chars().collect(),
            "@.@@@.@@@@".chars().collect(),
            ".@@@@@@@@.".chars().collect(),
            "@.@.@@@.@.".chars().collect(),
        ];

        assert_eq!(part1(grid), 13);
    }

    #[test]
    fn test_part2_cases() {
        let grid: Vec<Vec<char>> = vec![
            "..@@.@@@@.".chars().collect(),
            "@@@.@.@.@@".chars().collect(),
            "@@@@@.@.@@".chars().collect(),
            "@.@@@@..@.".chars().collect(),
            "@@.@@@@.@@".chars().collect(),
            ".@@@@@@@.@".chars().collect(),
            ".@.@.@.@@@".chars().collect(),
            "@.@@@.@@@@".chars().collect(),
            ".@@@@@@@@.".chars().collect(),
            "@.@.@@@.@.".chars().collect(),
        ];

        assert_eq!(part2(grid), 43);
    }
}