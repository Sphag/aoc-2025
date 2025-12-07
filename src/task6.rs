use std::fs::read_to_string;


fn part1(problems: Vec<(char, Vec<Vec<char>>)>) -> u64 {
    let mut grand_total: u64 = 0;
    for problem in problems {
        let mut cur_problem_result = match problem.0 {
            '+' => 0,
            '*' => 1,
            _ => { assert!(false, "Unknown operation {}", problem.0); 0 }
        };
        for number in problem.1 {
            cur_problem_result = match problem.0 {
                '+' => cur_problem_result + number.into_iter().filter(|x| *x != ' ').collect::<String>().parse::<u64>().unwrap(),
                '*' => cur_problem_result * number.into_iter().filter(|x| *x != ' ').collect::<String>().parse::<u64>().unwrap(),
                _ => { assert!(false, "Unknown operation {}", problem.0); cur_problem_result }
            };
        }
        grand_total += cur_problem_result;
    }

    grand_total
}


pub fn execute_part1_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let lines: Vec<Vec<char>> = file_input.lines().map(|x| x.chars().collect()).collect();

    let mut problems: Vec<(char, Vec<Vec<char>>)> = Vec::new();
    let mut last_pronblem_idx_in_str: Option<usize> = None;
    for mut i in 0 .. lines[lines.len()-1].len() {
        if lines[lines.len()-1][i] != ' ' || i == lines[lines.len()-1].len() -1 {
            if last_pronblem_idx_in_str.is_some() {
                let mut problem_numbers: Vec<Vec<char>> = Vec::new();
                if i == lines[lines.len()-1].len() -1 {
                    i += 2;
                }
                for j in 0 .. lines.len() - 1 {
                    problem_numbers.push(lines[j][last_pronblem_idx_in_str.unwrap() .. i-1].iter().copied().collect());
                }
                problems.push((lines[lines.len()-1][last_pronblem_idx_in_str.unwrap()], problem_numbers));
            }
            last_pronblem_idx_in_str = Some(i);
        }
    }
    println!("task6::part1 - result is {}", part1(problems));
}


fn part2(problems: Vec<(char, Vec<Vec<char>>)>) -> u64 {
    let mut grand_total: u64 = 0;
    for problem in problems {
        let mut cur_problem_result = match problem.0 {
            '+' => 0,
            '*' => 1,
            _ => { assert!(false, "Unknown operation {}", problem.0); 0 }
        };

        let mut vertical_numbers: Vec<Vec<char>> = Vec::new();
        vertical_numbers.resize(problem.1[0].len(), Vec::new());
        for number in problem.1 {
            for (i, digit) in number.iter().enumerate() {
                if *digit != ' ' {
                    vertical_numbers[i].push(*digit);
                }
            }
        }

        for number in vertical_numbers {
            cur_problem_result = match problem.0 {
                '+' => cur_problem_result + number.into_iter().filter(|x| *x != ' ').collect::<String>().parse::<u64>().unwrap(),
                '*' => cur_problem_result * number.into_iter().filter(|x| *x != ' ').collect::<String>().parse::<u64>().unwrap(),
                _ => { assert!(false, "Unknown operation {}", problem.0); cur_problem_result }
            };
        }

        grand_total += cur_problem_result;
    }

    grand_total
}


pub fn execute_part2_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let lines: Vec<Vec<char>> = file_input.lines().map(|x| x.chars().collect()).collect();

    let mut problems: Vec<(char, Vec<Vec<char>>)> = Vec::new();
    let mut last_pronblem_idx_in_str: Option<usize> = None;
    for mut i in 0 .. lines[lines.len()-1].len() {
        if lines[lines.len()-1][i] != ' ' || i == lines[lines.len()-1].len() -1 {
            if last_pronblem_idx_in_str.is_some() {
                let mut problem_numbers: Vec<Vec<char>> = Vec::new();
                if i == lines[lines.len()-1].len() -1 {
                    i += 2;
                }
                for j in 0 .. lines.len() - 1 {
                    problem_numbers.push(lines[j][last_pronblem_idx_in_str.unwrap() .. i-1].iter().copied().collect());
                }
                problems.push((lines[lines.len()-1][last_pronblem_idx_in_str.unwrap()], problem_numbers));
            }
            last_pronblem_idx_in_str = Some(i);
        }
    }
    println!("task6::part2 - result is {}", part2(problems));
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_cases() {
        let problems: Vec<(char, Vec<Vec<char>>)> = vec![
            ('*', vec!["123".chars().collect(), " 45".chars().collect(), "  6".chars().collect()]),
            ('+', vec!["328".chars().collect(), "64 ".chars().collect(), "98 ".chars().collect()]),
            ('*', vec![" 51".chars().collect(), "387".chars().collect(), "215".chars().collect()]),
            ('+', vec!["64 ".chars().collect(), "23 ".chars().collect(), "314".chars().collect()]),
        ];

        assert_eq!(part1(problems), 4277556);
    }

    #[test]
    fn test_part2_cases() {
        let problems: Vec<(char, Vec<Vec<char>>)> = vec![
            ('*', vec!["123".chars().collect(), " 45".chars().collect(), "  6".chars().collect()]),
            ('+', vec!["328".chars().collect(), "64 ".chars().collect(), "98 ".chars().collect()]),
            ('*', vec![" 51".chars().collect(), "387".chars().collect(), "215".chars().collect()]),
            ('+', vec!["64 ".chars().collect(), "23 ".chars().collect(), "314".chars().collect()]),
        ];

        assert_eq!(part2(problems), 3263827);
    }
}