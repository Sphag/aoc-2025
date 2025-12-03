use std::fs::File;
use std::io::{BufReader, BufRead};


fn part1(rotations: Vec<i32>) -> i32 {
    let mut dial: i32 = 50;
    let mut password = 0;
    for rotation in rotations {
        dial = (dial + rotation + 100) % 100;
        if dial == 0 {
            password += 1;
        }
    }
    password
}

pub fn execute_part1_from_input(input: &str) {
    let file = File::open(input).expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut rotations: Vec<i32> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut rotation: i32 = line.as_str()[1..].parse().expect("Failed to parse number");
                let line_chars: Vec<char> = line.chars().collect();
                match line_chars[0] {
                    'L' => rotation *= -1,
                    'R' => rotation *= 1,
                    _ => assert!(false, "First char should be either R or L but found {}", line_chars[0])
                }
                rotations.push(rotation);
            },
            Err(line) => assert!(false, "Failed to parse {}", line.to_string())
        }
    }

    println!("task1::part1 - result is {}", part1(rotations));
}


fn part2(rotations: Vec<i32>) -> i32 {
    let mut dial: i32 = 50;
    let mut password = 0;

    for mut rotation in rotations {
        password += (rotation / 100).abs();
        rotation = rotation % 100;

        if dial != 0 && (dial + rotation >= 100 || dial + rotation <= 0) {
            password += 1;
        }

        dial = (dial + rotation + 100) % 100;
    }
    password
}


pub fn execute_part2_from_input(input: &str) {
    let file = File::open(input).expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut rotations: Vec<i32> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut rotation: i32 = line.as_str()[1..].parse().expect("Failed to parse number");
                let line_chars: Vec<char> = line.chars().collect();
                match line_chars[0] {
                    'L' => rotation *= -1,
                    'R' => rotation *= 1,
                    _ => assert!(false, "First char should be either R or L but found {}", line_chars[0])
                }
                rotations.push(rotation);
            },
            Err(line) => assert!(false, "Failed to parse {}", line.to_string())
        }
    }

    println!("task1::part2 - result is {}", part2(rotations));
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_cases() {
        let test_rotations: Vec<i32> = vec![
            -68,
            -30,
            48,
            -5,
            60,
            -55,
            -1,
            -99,
            14,
            -82
        ];

        assert_eq!(part1(test_rotations), 3);
    }

        #[test]
    fn test_part2_cases() {
        let test_rotations: Vec<i32> = vec![
            -68,
            -30,
            48,
            -5,
            60,
            -55,
            -1,
            -99,
            14,
            -82
        ];

        assert_eq!(part2(test_rotations), 6);
    }
}