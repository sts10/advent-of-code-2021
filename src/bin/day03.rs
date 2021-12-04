use advent_of_code_2021::read_by_line;
// use advent_of_code_2021::split_and_vectorize;

fn main() {
    let file_name = "inputs/day03.txt";
    let puzzle_input: Vec<String> = read_by_line(file_name).expect("Error reading file by line");

    match solve_part_one(&puzzle_input) {
        Some(ans) => println!("Answer to part one is: {}", ans),
        None => println!("No answer found for part one"),
    }
    match solve_part_two(&puzzle_input) {
        Some(ans) => println!("Answer to part two is: {}", ans),
        None => println!("No answer found for part two"),
    }
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

fn most_common_in_column(column_index: usize, binary_codes: &[String]) -> char {
    let mut zeros_count = 0;
    let mut ones_count = 0;
    for code in binary_codes {
        let this_digit = code.chars().nth(column_index).unwrap();
        if this_digit == '0' {
            zeros_count += 1;
        } else if this_digit == '1' {
            ones_count += 1;
        } else {
            panic!("Not a 1 or 0");
        }
    }
    if zeros_count > ones_count {
        '0'
    } else {
        // if equal, we want to return a '1'
        '1'
    }
}

fn least_common_in_column(column_index: usize, binary_codes: &[String]) -> char {
    let mut zeros_count = 0;
    let mut ones_count = 0;
    for code in binary_codes {
        let this_digit = code.chars().nth(column_index).unwrap();
        if this_digit == '0' {
            zeros_count += 1;
        } else if this_digit == '1' {
            ones_count += 1;
        } else {
            eprintln!("Not a 1 or 0");
            return '2';
        }
    }
    // if equal, we want to return a '0'
    if zeros_count <= ones_count {
        '0'
    } else {
        '1'
    }
}

fn binary_to_decimal(binary_code: &str) -> usize {
    usize::from_str_radix(binary_code, 2).unwrap()
}

fn solve_part_two(binary_codes: &[String]) -> Option<usize> {
    let oxygen_generator_rating = match get_oxygen_generator_rating(binary_codes) {
        Some(rating) => rating,
        None => panic!("Did not get to 1 oxygen code..."),
    };
    let co2_scrubber_rating = &get_co2_scrubber_rating(binary_codes).unwrap();
    Some(binary_to_decimal(&oxygen_generator_rating) * binary_to_decimal(co2_scrubber_rating))
}

fn get_oxygen_generator_rating(binary_codes: &[String]) -> Option<String> {
    let number_of_digits = binary_codes[0].len();
    let mut binary_codes_as_vec: Vec<String> = vec![];
    for code in binary_codes {
        binary_codes_as_vec.push(code.to_string());
    }
    for column_index in 0..number_of_digits {
        let most_common_in_this_column = most_common_in_column(column_index, &binary_codes_as_vec);
        binary_codes_as_vec
            .retain(|code| code.chars().nth(column_index).unwrap() == most_common_in_this_column);
        if binary_codes_as_vec.len() == 1 {
            return Some(binary_codes_as_vec[0].to_string());
        }
    }
    None
}

fn get_co2_scrubber_rating(binary_codes: &[String]) -> Option<String> {
    let number_of_digits = binary_codes[0].chars().count();
    let mut binary_codes_as_vec: Vec<String> = vec![];
    for code in binary_codes {
        binary_codes_as_vec.push(code.to_string());
    }
    for column_index in 0..number_of_digits {
        let least_common_in_column = least_common_in_column(column_index, &binary_codes_as_vec);
        binary_codes_as_vec
            .retain(|code| code.chars().nth(column_index).unwrap() == least_common_in_column);
        if binary_codes_as_vec.len() == 1 {
            return Some(binary_codes_as_vec[0].to_string());
        }
    }
    None
}

#[test]
fn can_solve_day_3_part_one() {
    let file_name = "inputs/day03.txt";
    let input_data: Vec<String> = read_by_line(file_name).unwrap();
    assert_eq!(solve_part_one(&input_data), Some(3277364));
}

#[test]
fn can_solve_day_3_part_two() {
    let file_name = "inputs/day03.txt";
    let input_data: Vec<String> = read_by_line(file_name).unwrap();
    assert_eq!(solve_part_two(&input_data), Some(5736383));
}
