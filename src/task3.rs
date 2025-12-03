use std::fs::read_to_string;


fn part1(banks: Vec<&str>) -> u64 {
    let mut total_joltage: u64 = 0;
    for bank in banks {
        let bank_batteries: Vec<u64> = bank.chars().map(|x| x.to_digit(10).unwrap() as u64).collect();

        let mut prefix_max: Vec<u64> = Vec::new();
        let mut suffix_max: Vec<u64> = Vec::new();

        prefix_max.resize(bank_batteries.len(), 0);
        suffix_max.resize(bank_batteries.len(), 0);

        prefix_max[0] = bank_batteries[0];
        suffix_max[bank_batteries.len() - 1] = bank_batteries[bank_batteries.len() - 1];

        for i in 1 .. bank_batteries.len() {
            prefix_max[i] = prefix_max[i - 1].max(bank_batteries[i]);
            suffix_max[bank_batteries.len() - 1 - i] = suffix_max[bank_batteries.len() - i].max(bank_batteries[bank_batteries.len() - 1 - i]);
        }

        let mut max_joltage: (u64, u64) = (0, 0);
        for i in 0 .. bank_batteries.len() - 1 {
            max_joltage = max_joltage.max((prefix_max[i], suffix_max[i + 1]));
        }

        total_joltage += 10 * max_joltage.0 + max_joltage.1;
    }

    total_joltage
}


pub fn execute_part1_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let banks: Vec<&str> = file_input.lines().collect();
    println!("task3::part1 - result is {}", part1(banks));
}


fn part2(banks: Vec<&str>) -> u64 {
    let mut total_joltage: u64 = 0;
    for bank in banks {
        let bank_batteries: Vec<u64> = bank.chars().map(|x| x.to_digit(10).unwrap() as u64).collect();

        let mut stack: Vec<u64> = Vec::new();
        let mut batteries_to_drop = bank_batteries.len() - 12;
        for battery in bank_batteries {
            while !stack.is_empty() && stack.last() < Some(&battery) && batteries_to_drop > 0 {
                stack.pop();
                batteries_to_drop -= 1;
            }
            stack.push(battery);
        }
        stack.truncate(12);

        let mut max_bank_joltage: u64 = 0;
        for i in 0 .. 12 {
            max_bank_joltage += stack[i] * 10_u64.pow(11 - i as u32);
        }
        total_joltage += max_bank_joltage;
    }

    total_joltage
}


pub fn execute_part2_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let banks: Vec<&str> = file_input.lines().collect();
    println!("task3::part2 - result is {}", part2(banks));
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_cases() {
        let banks: Vec<&str> = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(part1(banks), 357);
    }

    #[test]
    fn test_part2_cases() {
        let banks: Vec<&str> = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(part2(banks), 3121910778619);
    }
}