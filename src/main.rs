mod task1;
mod task2;


fn main() {
    task1::execute_part1_from_input("inputs/task1.txt");
    task1::execute_part2_from_input("inputs/task1.txt");

    task2::execute_part1_from_input("inputs/task2.txt");
    task2::execute_part2_from_input("inputs/task2.txt");
}