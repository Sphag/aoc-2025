mod task1;
mod task2;
mod task3;
mod task4;
mod task5;
mod task6;


fn main() {
    task1::execute_part1_from_input("inputs/task1.txt");
    task1::execute_part2_from_input("inputs/task1.txt");

    task2::execute_part1_from_input("inputs/task2.txt");
    task2::execute_part2_from_input("inputs/task2.txt");

    task3::execute_part1_from_input("inputs/task3.txt");
    task3::execute_part2_from_input("inputs/task3.txt");

    task4::execute_part1_from_input("inputs/task4.txt");
    task4::execute_part2_from_input("inputs/task4.txt");

    task5::execute_part1_from_input("inputs/task5.txt");
    task5::execute_part2_from_input("inputs/task5.txt");

    task6::execute_part1_from_input("inputs/task6.txt");
    task6::execute_part2_from_input("inputs/task6.txt");
}