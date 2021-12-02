use advent_of_code_2021::read_by_line;

fn main() {
    let file_name = "inputs/day01.txt";
    let depth_soundings: Vec<usize> = read_by_line(file_name).unwrap();
    match solve_part_one(&depth_soundings) {
        Some(ans) => println!("Answer to part one day 1 2021 is: {}", ans),
        None => println!("No answer found for part one"),
    }
    match solve_part_two(&depth_soundings) {
        Some(ans) => println!("Answer to part two is: {}", ans),
        None => println!("No answer found for part two"),
    }
}

fn solve_part_one(soundings: &[usize]) -> Option<usize> {
    let mut number_of_increases = 0;
    for i in 1..soundings.len() {
        if soundings[i] > soundings[i - 1] {
            number_of_increases += 1;
        }
    }
    Some(number_of_increases)
}

fn solve_part_two(soundings: &[usize]) -> Option<usize> {
    let mut number_of_increases = 0;
    for i in 1..soundings.len() {
        if sum_next_three(soundings, i) > sum_next_three(soundings, i - 1) {
            number_of_increases += 1;
        }
    }
    Some(number_of_increases)
}

fn sum_next_three(soundings: &[usize], index: usize) -> Option<usize> {
    if soundings.len() > index + 2 {
        Some(soundings[index] + soundings[index + 1] + soundings[index + 2])
    } else {
        None
    }
}

#[test]
fn can_solve_day_1_part_one() {
    let file_name = "inputs/day01.txt";
    let soundings: Vec<usize> = read_by_line(file_name).unwrap();
    assert_eq!(solve_part_one(&soundings), Some(1154));
}

#[test]
fn can_solve_day_1_part_two() {
    let file_name = "inputs/day01.txt";
    let soundings: Vec<usize> = read_by_line(file_name).unwrap();
    assert_eq!(solve_part_two(&soundings), Some(1127));
}
