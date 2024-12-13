use std::fs;

pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return contents;
}

pub fn get_number_of_lines(file_content: String) -> usize {
    let lines = file_content.split("\n");
    let lines_collection: Vec<_> = lines.clone().collect();
    let number_of_lines: usize = lines_collection.len();
    return number_of_lines;
}

pub fn convert_string_to_number(value: &str) -> i32 {
    return value.trim().parse().expect("Error: input is not a number");
}
