use std::fs::read_to_string;
use itertools::Itertools;


fn part1(junction_boxes: Vec<(u64, u64, u64)>, connections_limit: u64) -> u64 {
    let mut circuits: Vec<u64> = Vec::new();
    circuits.resize(junction_boxes.len(), 1);

    let mut junction_box_curcuit_idx: Vec<usize> = (0 .. circuits.len()).map(|x| x).collect();

    for _c in 0 .. connections_limit-1 {
        let mut nearest_junction_box_indices = (usize::max_value(), usize::max_value());
        let mut distance_to_nearest_junction_box = u64::max_value();
        for i in 0 .. junction_boxes.len() {
            for j in 0 .. junction_boxes.len() {
                if i != j && junction_box_curcuit_idx[i] != junction_box_curcuit_idx[j] {
                    let distance = (junction_boxes[i].0.max(junction_boxes[j].0) - junction_boxes[i].0.min(junction_boxes[j].0)) * (junction_boxes[i].0.max(junction_boxes[j].0) - junction_boxes[i].0.min(junction_boxes[j].0)) + 
                                    (junction_boxes[i].1.max(junction_boxes[j].1) - junction_boxes[i].1.min(junction_boxes[j].1)) * (junction_boxes[i].1.max(junction_boxes[j].1) - junction_boxes[i].1.min(junction_boxes[j].1)) + 
                                    (junction_boxes[i].2.max(junction_boxes[j].2) - junction_boxes[i].2.min(junction_boxes[j].2)) * (junction_boxes[i].2.max(junction_boxes[j].2) - junction_boxes[i].2.min(junction_boxes[j].2));
                    if distance < distance_to_nearest_junction_box {
                        nearest_junction_box_indices = (i, j);
                        distance_to_nearest_junction_box = distance;
                    }
                }
            }
        }
        circuits[junction_box_curcuit_idx[nearest_junction_box_indices.0]] += circuits[junction_box_curcuit_idx[nearest_junction_box_indices.1]];
        circuits[junction_box_curcuit_idx[nearest_junction_box_indices.1]] = 0;
        for j in 0 .. junction_box_curcuit_idx.len() {
            if junction_box_curcuit_idx[j] == junction_box_curcuit_idx[nearest_junction_box_indices.1] {
                junction_box_curcuit_idx[j] = junction_box_curcuit_idx[nearest_junction_box_indices.0];
            }
        }
    }

    circuits.iter().unique().fold(1, |acc, x| {
        if *x == 0 {
            acc
        } else {
            acc * *x
        }
    })
}


pub fn execute_part1_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let junction_boxes: Vec<(u64, u64, u64)> = file_input.lines().map(|x| x.split(',').map(|n| n.parse::<u64>().unwrap()).collect_tuple().unwrap()).collect();
    println!("task8::part1 - result is {}", part1(junction_boxes, 1000));
}


fn part2(junction_boxes: Vec<(u64, u64, u64)>, connections_limit: u64) -> u64 {
    let mut circuits: Vec<u64> = Vec::new();
    circuits.resize(junction_boxes.len(), 1);

    let mut junction_box_curcuit_idx: Vec<usize> = (0 .. circuits.len()).map(|x| x).collect();

    for _c in 0 .. connections_limit-1 {
        let mut nearest_junction_box_indices = (usize::max_value(), usize::max_value());
        let mut distance_to_nearest_junction_box = u64::max_value();
        for i in 0 .. junction_boxes.len() {
            for j in 0 .. junction_boxes.len() {
                if i != j && junction_box_curcuit_idx[i] != junction_box_curcuit_idx[j] {
                    let distance = (junction_boxes[i].0.max(junction_boxes[j].0) - junction_boxes[i].0.min(junction_boxes[j].0)) * (junction_boxes[i].0.max(junction_boxes[j].0) - junction_boxes[i].0.min(junction_boxes[j].0)) + 
                                    (junction_boxes[i].1.max(junction_boxes[j].1) - junction_boxes[i].1.min(junction_boxes[j].1)) * (junction_boxes[i].1.max(junction_boxes[j].1) - junction_boxes[i].1.min(junction_boxes[j].1)) + 
                                    (junction_boxes[i].2.max(junction_boxes[j].2) - junction_boxes[i].2.min(junction_boxes[j].2)) * (junction_boxes[i].2.max(junction_boxes[j].2) - junction_boxes[i].2.min(junction_boxes[j].2));
                    if distance < distance_to_nearest_junction_box {
                        nearest_junction_box_indices = (i, j);
                        distance_to_nearest_junction_box = distance;
                    }
                }
            }
        }
        circuits[junction_box_curcuit_idx[nearest_junction_box_indices.0]] += circuits[junction_box_curcuit_idx[nearest_junction_box_indices.1]];
        circuits[junction_box_curcuit_idx[nearest_junction_box_indices.1]] = 0;
        for j in 0 .. junction_box_curcuit_idx.len() {
            if junction_box_curcuit_idx[j] == junction_box_curcuit_idx[nearest_junction_box_indices.1] {
                junction_box_curcuit_idx[j] = junction_box_curcuit_idx[nearest_junction_box_indices.0];
            }
        }
    }

    circuits.iter().unique().fold(1, |acc, x| {
        if *x == 0 {
            acc
        } else {
            acc * *x
        }
    })
}


pub fn execute_part2_from_input(input: &str) {
    let file_input = read_to_string(input).expect("Couldn't read file");
    let junction_boxes: Vec<(u64, u64, u64)> = file_input.lines().map(|x| x.split(',').map(|n| n.parse::<u64>().unwrap()).collect_tuple().unwrap()).collect();
    println!("task8::part2 - result is {}", part2(junction_boxes, 1000));
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

        assert_eq!(part2(junction_boxes, 10), 40);
    }
}