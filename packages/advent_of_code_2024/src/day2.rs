use super::utils::read_file;
use super::utils::convert_string_to_number;

pub fn first_puzzle() {
    get_matrix()
}

// pub fn second_puzzle() { }

fn get_matrix() {
    let contents = read_file("assets/day-2.txt");
    let lines = contents.split("\n");

    let mut safe_reports: i32 = 0;

    for line in lines {
        if is_line_safe(line) {
            safe_reports += 1;
        }
    }
    println!("safe reports: {}", safe_reports);
}

fn is_line_safe(line: &str) -> bool {
    let mut is_safe = false;
    let values: Vec<_> = line.split(" ").collect();
    let length: usize = values.len();
    let mut numbers = vec![0; length];
    for (i, value) in values.into_iter().enumerate() {
        numbers[i] = convert_string_to_number(value);
    }
    if is_line_decreasing(numbers.clone()) || is_line_increasing(numbers.clone()) {
        if is_distance_safe(numbers.clone()) {
            is_safe = true;
        }
    }
    return is_safe;
}

fn is_line_increasing(numbers: Vec<i32>) -> bool {
    let mut is_increasing = true;
    for (i, number) in numbers.clone().into_iter().enumerate() {
        if i != 0 && numbers[i - 1] > number {
            is_increasing = false;
        }
    }
    return is_increasing;
}

fn is_line_decreasing(numbers: Vec<i32>) -> bool {
    let mut is_decreasing = true;
    for (i, number) in numbers.clone().into_iter().enumerate() {
        if i != 0 && numbers[i - 1] < number {
            is_decreasing = false;
        }
    }
    return is_decreasing;
}

fn is_distance_safe(numbers: Vec<i32>) -> bool {
    let mut is_safe = true;
    for (i, number) in numbers.clone().into_iter().enumerate() {
        if i != 0 {
            let distance = numbers[i - 1] - number;
            if distance.abs() > 3 || distance == 0 {
                is_safe = false;
            }
        }
    }
    return is_safe;
}