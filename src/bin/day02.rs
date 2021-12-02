use advent_of_code_2021::read_by_line;
use advent_of_code_2021::split_and_vectorize;

fn main() {
    let file_name = "inputs/day02.txt";
    let puzzle_input: Vec<String> = read_by_line(file_name).unwrap();
    match solve_part_one(&puzzle_input) {
        Some(ans) => println!("Answer to part one is: {}", ans),
        None => println!("No answer found for part one"),
    }
    match solve_part_two(&puzzle_input) {
        Some(ans) => println!("Answer to part two is: {}", ans),
        None => println!("No answer found for part two"),
    }
}

fn solve_part_one(instructions: &[String]) -> Option<isize> {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for instruction in instructions {
        let instruction_as_vec = split_and_vectorize(instruction, " ");
        let direction = instruction_as_vec[0].trim();
        let amount = instruction_as_vec[1].parse::<isize>().unwrap();
        if direction == "forward" {
            horizontal_position += amount
        } else if direction == "down" {
            depth += amount;
        } else if direction == "up" {
            depth -= amount;
        }
    }
    Some(horizontal_position * depth)
}

fn solve_part_two(instructions: &[String]) -> Option<isize> {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in instructions {
        let instruction_as_vec = split_and_vectorize(instruction, " ");
        let direction = instruction_as_vec[0].trim();
        let amount = instruction_as_vec[1].parse::<isize>().unwrap();
        if direction == "down" {
            aim += amount;
        } else if direction == "up" {
            aim -= amount;
        } else if direction == "forward" {
            horizontal_position += amount;
            depth = depth + aim * amount;
        }
    }
    Some(horizontal_position * depth)
}

#[test]
fn can_solve_day_2_part_one() {
    let file_name = "inputs/day02.txt";
    let input_data: Vec<String> = read_by_line(file_name).unwrap();
    assert_eq!(solve_part_one(&input_data), Some(1635930));
}

#[test]
fn can_solve_day_2_part_two() {
    let file_name = "inputs/day02.txt";
    let input_data: Vec<String> = read_by_line(file_name).unwrap();
    assert_eq!(solve_part_two(&input_data), Some(1781819478));
}
