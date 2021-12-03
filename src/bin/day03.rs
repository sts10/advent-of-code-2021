use advent_of_code_2021::read_by_line;
use advent_of_code_2021::split_and_vectorize;

fn main() {
    let file_name = "inputs/day03.txt";
    let puzzle_input: Vec<String> = read_by_line(file_name).expect("Error reading file by line");

    // solve_part_one(&puzzle_input);
    match solve_part_one(&puzzle_input) {
        Some(ans) => println!("Answer to part one is: {}", ans),
        None => println!("No answer found for part one"),
    }
    // match solve_part_two(&puzzle_input) {
    //     Some(ans) => println!("Answer to part two is: {}", ans),
    //     None => println!("No answer found for part two"),
    // }
}

fn solve_part_one(binary_codes: &[String]) -> Option<usize> {
    let number_of_digits = binary_codes[0].chars().count();
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for place_index in 0..number_of_digits {
        let mut zeros_count = 0;
        let mut ones_count = 0;
        for code in binary_codes {
            let this_digit = code.chars().nth(place_index).unwrap();
            if this_digit == '0' {
                zeros_count += 1;
            } else if this_digit == '1' {
                ones_count += 1;
            } else {
                eprintln!("Not a 1 or 0");
                return None;
            }
        }
        if zeros_count > ones_count {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }
    Some(binary_to_decimal(&gamma_rate) * binary_to_decimal(&epsilon_rate))
}

fn binary_to_decimal(binary_code: &str) -> usize {
    usize::from_str_radix(binary_code, 2).unwrap()
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
