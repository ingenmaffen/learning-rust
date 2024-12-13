use super::utils::read_file;
use super::utils::get_number_of_lines;
use super::utils::convert_string_to_number;

pub fn first_puzzle() {
    let arrays = get_arrays();
    let mut left_side_numbers = arrays[0].clone();
    let mut right_side_numbers = arrays[1].clone();

    left_side_numbers.sort();
    right_side_numbers.sort();

    let mut diff: i32 = 0;

    for i in 0..left_side_numbers.len() {
        let value = left_side_numbers[i] - right_side_numbers[i];
        diff += value.abs();
    }

    println!("{}", diff);
}

pub fn second_puzzle() {
    let arrays = get_arrays();
    let left_side_numbers = arrays[0].clone();
    let right_side_numbers = arrays[1].clone();

    let mut result: i32 = 0;

    for value in left_side_numbers {
        for other_value in right_side_numbers.clone().into_iter() {
            if (value == other_value) {
                result += value;
            }
        }
    }

    println!("{}", result);
}

fn get_arrays() -> [Vec<i32>; 2] {
    let contents = read_file("assets/day-1-1.txt");
    let lines = contents.split("\n");
    let number_of_lines = get_number_of_lines(contents.clone());
    let mut left_side_numbers = vec![0; number_of_lines];
    let mut right_side_numbers = vec![0; number_of_lines];

    for (i, line) in lines.enumerate() {
        let number_as_strings: Vec<_> = line.split("   ").collect();
        left_side_numbers[i] = convert_string_to_number(number_as_strings[0]);
        right_side_numbers[i] = convert_string_to_number(number_as_strings[1]);
    }
    return [left_side_numbers, right_side_numbers];
}