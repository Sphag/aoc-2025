use std::fs::read_to_string;
use itertools::{Itertools, iproduct};
use std::collections::HashMap;


fn distance(a: (u64, u64, u64), b: (u64, u64, u64)) -> u64 {
    (a.0.max(b.0) - a.0.min(b.0)) * (a.0.max(b.0) - a.0.min(b.0)) +
    (a.1.max(b.1) - a.1.min(b.1)) * (a.1.max(b.1) - a.1.min(b.1)) +
    (a.2.max(b.2) - a.2.min(b.2)) * (a.2.max(b.2) - a.2.min(b.2))
}


fn part1(junction_boxes: Vec<(u64, u64, u64)>, connections_limit: u64) -> u64 {
    let mut circuits: Vec<u64> = Vec::new();
    circuits.resize(junction_boxes.len(), 1);

    let mut circuits_ids: HashMap<(u64, u64, u64), usize> = HashMap::new();
    for (idx, junction_box) in junction_boxes.iter().enumerate() {
        circuits_ids.insert(*junction_box, idx);
    }

    let connections: Vec<((u64, u64, u64), (u64, u64, u64))> = iproduct!(junction_boxes.clone(), junction_boxes.clone()).sorted_by(|a, b| {
        let distance_a = distance(a.0, a.1);
        let distance_b = distance(b.0, b.1);
        distance_a.cmp(&distance_b)
    }).filter(|x| distance(x.0, x.1) > 0).step_by(2).collect();

    let mut connections_made = 0;
    for (connection_a, connection_b) in connections {
        if connections_made == connections_limit {
            break;
        }
        let circuit_id_a = *circuits_ids.get(&connection_a).unwrap();
        let circuit_id_b = *circuits_ids.get(&connection_b).unwrap();
        if circuit_id_a == circuit_id_b {
            connections_made += 1;
            continue;
        }
        circuits[circuit_id_a] += circuits[circuit_id_b];
        circuits[circuit_id_b] = 0;
        for (_, circuit_id) in circuits_ids.iter_mut() {
            if *circuit_id == circuit_id_b {
                *circuit_id = circuit_id_a;
            }
        }
        connections_made += 1;
    }
    circuits.iter().unique().filter(|&x| *x != 0).sorted_by(|a, b| b.cmp(a)).take(3).fold(1, |acc, x| acc * x)
}


pub fn execute_part1_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let junction_boxes: Vec<(u64, u64, u64)> = file_input.lines().map(|x| x.split(',').map(|n| n.parse::<u64>().unwrap()).collect_tuple().unwrap()).collect();
    println!("task8::part1 - result is {}", part1(junction_boxes, 1000));
}


fn part2(junction_boxes: Vec<(u64, u64, u64)>) -> u64 {
    let mut circuits_ids: HashMap<(u64, u64, u64), usize> = HashMap::new();
    for (idx, junction_box) in junction_boxes.iter().enumerate() {
        circuits_ids.insert(*junction_box, idx);
    }

    let connections: Vec<((u64, u64, u64), (u64, u64, u64))> = iproduct!(junction_boxes.clone(), junction_boxes.clone()).sorted_by(|a, b| {
        let distance_a = distance(a.0, a.1);
        let distance_b = distance(b.0, b.1);
        distance_a.cmp(&distance_b)
    }).filter(|x| distance(x.0, x.1) > 0).step_by(2).collect();

    let mut active_circuits = junction_boxes.len();
    let mut result: u64 = 0;
    while active_circuits > 1 {
        for (connection_a, connection_b) in &connections {
            let circuit_id_a = *circuits_ids.get(&connection_a).unwrap();
            let circuit_id_b = *circuits_ids.get(&connection_b).unwrap();
            if circuit_id_a == circuit_id_b {
                continue;
            }
            for (_, circuit_id) in circuits_ids.iter_mut() {
                if *circuit_id == circuit_id_b {
                    *circuit_id = circuit_id_a;
                }
            }
            result = connection_a.0 * connection_b.0;
            active_circuits -= 1;
        }
    }
    result
}


pub fn execute_part2_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let junction_boxes: Vec<(u64, u64, u64)> = file_input.lines().map(|x| x.split(',').map(|n| n.parse::<u64>().unwrap()).collect_tuple().unwrap()).collect();
    println!("task8::part2 - result is {}", part2(junction_boxes));
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_cases() {
        let junction_boxes: Vec<(u64, u64, u64)> = vec![
            (162,817,812),
            (57, 618,57),
            (926,362,562),
            (592,479,942),
            (352,342,322),
            (466,668,158),
            (542,29,236),
            (431,825,988),
            (739,652,466),
            (52, 472,668),
            (216,146,977),
            (819,987,18),
            (117,168,532),
            (825,96, 715),
            (346,949,466),
            (972,615,88),
            (941,993,342),
            (862,61, 35),
            (984,92, 344),
            (425,692,689),
        ];

        assert_eq!(part1(junction_boxes, 10), 40);
    }

    #[test]
    fn test_part2_cases() {
        let junction_boxes: Vec<(u64, u64, u64)> = vec![
            (162,817,812),
            (57, 618,57),
            (926,362,562),
            (592,479,942),
            (352,342,322),
            (466,668,158),
            (542,29,236),
            (431,825,988),
            (739,652,466),
            (52, 472,668),
            (216,146,977),
            (819,987,18),
            (117,168,532),
            (825,96, 715),
            (346,949,466),
            (972,615,88),
            (941,993,342),
            (862,61, 35),
            (984,92, 344),
            (425,692,689),
        ];

        assert_eq!(part2(junction_boxes), 25272);
    }
}